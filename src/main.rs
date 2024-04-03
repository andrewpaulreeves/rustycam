use ndarray::{Array, Array2};
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::{Normal, Uniform};
use core::time;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
// use std::time::Instant;
use std::thread;
use std::option;

pub struct Camera {
    n_rows: usize,
    n_cols: usize,
    frame_number: Arc<AtomicU64>,
    acquiring: Arc<AtomicBool>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
    frame_buffer: Arc<Mutex<Array2<u16>>>,
}

impl Camera {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
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
        }
    }

    pub fn start_acquisition(&mut self) {
        println!("Start Acquisition");
        
        // Get some references to data in self
        let fn_ref = Arc::clone(&self.frame_number);
        let acq_ref = Arc::clone(&self.acquiring);
        let fb_ref = Arc::clone(&self.frame_buffer);
        
        // Set acquiring to True until its set otherwise
        acq_ref.store(true, Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move ||{
            while acq_ref.load(Ordering::Relaxed) {
                thread::sleep(time::Duration::from_secs(1));

                let fr = fn_ref.load(Ordering::Relaxed);
                fn_ref.store(fr+1, Ordering::Relaxed);

                let mut frame_buffer = fb_ref.lock().unwrap();
                frame_buffer[(0, 0)] = fr as u16;
            }
        }));
    }

    pub fn stop_acquisition(&mut self) {
        println!("Stopping Acquisition...");
        self.acquiring.store(false, Ordering::Relaxed);
        println!("Stopping Acquisition...Done");
    }
}

fn main() {
    println!("Hello, Camera!");
    println!("Init Camera...");

    let rows = 256;
    let cols = 320;

    let mut cam = Camera::new(rows, cols);
    println!("Init Camera...Done");
    
    // println!("Cam Acquiring:    {}", cam.acquiring);
    // println!("Frame Number:     {}", cam.frame_number);

    cam.start_acquisition();

    println!("Started Acquisition...");
    println!("Wait 5 seconds...");

    for _ in 0..5 {
        thread::sleep(time::Duration::from_secs(1));
        let fr = cam.frame_number.load(Ordering::Relaxed);
        println!("Frame Number: {}", fr);

        let fb_ref = cam.frame_buffer.clone();
        let frame_buf = fb_ref.lock().unwrap();
        println!("Frame Buf {}", frame_buf);

    }
    println!("Done!");

    cam.stop_acquisition();


}

