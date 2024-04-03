// use ndarray::Array;
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::{Normal, Uniform};
use core::time;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
// use std::time::Instant;
use std::thread;
use std::option;

pub struct Camera {
    frame_number: AtomicU64,
    acquiring: AtomicBool,
    thread_handle: option::Option<thread::JoinHandle<()>>,
}

impl Camera {
    pub fn new() -> Self {
        Self{
            frame_number: AtomicU64::new(0),
            acquiring: AtomicBool::new(false),
            thread_handle: None
        }
    }

    pub fn start_acquisition(&mut self) {
        println!("Start Acquisition");
        self.acquiring.store(true, Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move ||{
            while self.acquiring.load(Ordering::Relaxed) {
                thread::sleep(time::Duration::from_secs(1));
                let fr = self.frame_number.load(Ordering::Relaxed);
                self.frame_number.store(fr+1, Ordering::Relaxed);
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
    let mut cam = Camera::new();
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




    // let rows = 5120;
    // let cols = 6400;
    // let n_iters = 100;

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

