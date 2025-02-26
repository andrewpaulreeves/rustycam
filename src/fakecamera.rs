use ndarray::Array2;
use core::time;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::option;
use rand_distr::{Distribution, Normal};
use rand::rng;

pub struct Camera {
    n_rows: usize,
    n_cols: usize,
    frame_number: Arc<AtomicU64>,
    acquiring: Arc<AtomicBool>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
    frame_buffer: Arc<Mutex<Array2<u16>>>,
    e_read_noise: f32,
    frame_rate: f32,
}

impl Camera {
    pub fn new(n_rows: usize, n_cols: usize, e_read_noise: f32, frame_rate: f32) -> Self {
        // let frame_shape: Dim::<u32>::
        let frame_shape = (n_rows, n_cols);
        let frame_buffer = Arc::new(Mutex::new(
            Array2::<u16>::zeros(frame_shape)
        ));
        Self{
            n_rows: n_rows,
            n_cols: n_cols,
            frame_number: Arc::new(AtomicU64::new(0)),
            acquiring: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
            frame_buffer: frame_buffer,
            e_read_noise: e_read_noise,
            frame_rate: frame_rate,
        }
    }

    pub fn start_acquisition(&mut self) {
        println!("Start Acquisition");
        
        // Get some references to data in self
        let fn_ref = Arc::clone(&self.frame_number);
        let acq_ref = Arc::clone(&self.acquiring);
        let fb_ref = Arc::clone(&self.frame_buffer);
        
        let n_rows = self.n_rows;
        let n_cols = self.n_cols;

        // Set acquiring to True until its set otherwise
        acq_ref.store(true, Ordering::Relaxed);

        let frame_rate = self.frame_rate;
        let e_read_noise = self.e_read_noise;

        self.thread_handle = option::Option::Some(std::thread::spawn(move ||{
            let mut e_read_rng = rng();
            let normal = Normal::new(0.0, e_read_noise).unwrap();

            let mut frame_buffer_tmp = Array2::<u16>::zeros((n_rows, n_cols));
            while acq_ref.load(Ordering::Relaxed) {
                if (frame_rate != 0.0) {
                    thread::sleep(time::Duration::from_millis((1000.0 / frame_rate) as u64));
                }

                let fr = fn_ref.load(Ordering::Relaxed);
                fn_ref.store(fr+1, Ordering::Relaxed);

                // Add read noise
                for i in frame_buffer_tmp.iter_mut() {
                    *i = normal.sample(&mut e_read_rng) as u16;
                }

                // copy tmp buffer to shared buffer
                let mut frame_buffer = fb_ref.lock().unwrap();
                frame_buffer.assign(&frame_buffer_tmp)

            }
        }));
    }

    pub fn stop_acquisition(&mut self) {
        println!("Stopping Acquisition...");
        self.acquiring.store(false, Ordering::Relaxed);
        println!("Stopping Acquisition...Done");
    }

    pub fn get_frame(&self) -> Array2<u16> {
        let fb_ref = self.frame_buffer.clone();
        let frame_buffer = fb_ref.lock().unwrap();
        frame_buffer.clone()
    }   

    pub fn get_frame_number(&self) -> u64 {
        self.frame_number.load(Ordering::Relaxed)
    }
}
