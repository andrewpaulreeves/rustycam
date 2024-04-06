use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::option;
use std::time;

pub struct Camera {
    frame_number: Arc<AtomicU64>,
    acquiring: Arc<AtomicBool>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
}

impl Camera {
    pub fn new() -> Self {
        Self{
            frame_number: Arc::new(AtomicU64::new(0)),
            acquiring: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn start_acquisition(&mut self) {
        println!("Start Acquisition");
        
        // Get some references to data in self
        let fn_ref = Arc::clone(&self.frame_number);
        let acq_ref = Arc::clone(&self.acquiring);
        
        // Set acquiring to True until its set otherwise
        acq_ref.store(true, Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move ||{
            while acq_ref.load(Ordering::Relaxed) {
                // thread::sleep(time::Duration::from_secs(1));

                let fr = fn_ref.load(Ordering::Relaxed);
                fn_ref.store(fr+1, Ordering::Relaxed);
            }
        }));
    }

    pub fn stop_acquisition(&mut self) {
        self.acquiring.store(false, Ordering::Relaxed);
    }
}

fn main() {
    let mut cam = Camera::new();
    cam.start_acquisition();

    for i in 0..5 {
        thread::sleep(time::Duration::from_secs(1));
        let fr = cam.frame_number.load(Ordering::Relaxed);
        println!("Iters per second: {}", ((fr as f32) / ((i+1) as f32)));
    }
    cam.stop_acquisition();


}
