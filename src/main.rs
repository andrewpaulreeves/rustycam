use ndarray::{Array, Array2d};
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::{Normal, Uniform};
use core::time;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
// use std::time::Instant;
use std::thread;
use std::option;

pub struct Camera {
    n_rows: u32,
    n_cols: u32,
    frame_number: Arc<AtomicU64>,
    acquiring: Arc<AtomicBool>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
    frame_buffer: Arc<Mutex<Vec<u16>>>,
}

impl Camera {
    pub fn new(n_rows: u32, n_cols: u32) -> Self {
        let frame_buffer = Arc::new(Mutex::new(
            Array2d
        ));
        Self{
            n_rows: n_rows,
            n_cols: n_cols,
            frame_number: Arc::new(AtomicU64::new(0)),
            acquiring: Arc::new(AtomicBool::new(false)),
            thread_handle: None

        }
    }

    pub fn start_acquisition(&mut self) {
        println!("Start Acquisition");
        
        let fn_ref = Arc::clone(&self.frame_number);
        let acq_ref = Arc::clone(&self.acquiring);

        acq_ref.store(true, Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move ||{
            while acq_ref.load(Ordering::Relaxed) {
                thread::sleep(time::Duration::from_secs(1));

                let fr = fn_ref.load(Ordering::Relaxed);
                fn_ref.store(fr+1, Ordering::Relaxed);
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

    let rows = 512;
    let cols = 640;

    let mut cam = Camera::new(rows, cols);
    println!("Init Camera...Done");
    
    // println!("Cam Acquiring:    {}", cam.acquiring);
    // println!("Frame Number:     {}", cam.frame_number);

    cam.start_acquisition();

    println!("Started Acquisition...");
    println!("Wait 5 seconds...");
    thread::sleep(time::Duration::from_secs(5));
    println!("Done!");

    cam.stop_acquisition();
    let fr = cam.frame_number.load(Ordering::Relaxed);
    println!("Frame Number: {}", fr);

    // let t_start = Instant::now();

    // let mut noise = Array::random(
    //     (rows, cols), Uniform::new(0., 10.0));

    // for i in 0..n_iters {
    //     noise = Array::random(
    //         (rows, cols), Uniform::new(0., 10.0));
        
    //     noise[[0,0]] = i as f64;
    //     }
    // let t_elapsed = t_start.elapsed().as_millis() / n_iters;
    
    // println!("noise vector {}", noise);
    // println!("Time per iter: {} ms", t_elapsed);

}

