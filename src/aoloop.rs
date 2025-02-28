
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::option;
use log::{trace, debug, info, warn};

use crate::fakecamera::Camera;
use crate::wfs::ShackHartmann;
use crate::fakedm::DM;
use crate::controller::IntegratorController;

pub struct AOLoop {
    cameras: Arc<Vec<Camera>>,
    wfs: Arc<Vec<ShackHartmann>>,
    controller: Arc<Mutex<IntegratorController>>,
    dms: Arc<Mutex<Vec<DM>>>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
    loop_running: Arc<AtomicBool>,
    iteration_number: Arc<AtomicU64>,
}


impl AOLoop {
    pub fn new(cameras: Vec<Camera>, wfs: Vec<ShackHartmann>, controller: IntegratorController, dms: Vec<DM>) -> Self {
        let loop_running = Arc::new(AtomicBool::new(false));
        let iteration_number = Arc::new(AtomicU64::new(0));
        Self {
            cameras: Arc::new(cameras),
            wfs: Arc::new(wfs),
            controller: Arc::new(Mutex::new(controller)),
            dms: Arc::new(Mutex::new(dms)),
            thread_handle: None,
            loop_running: loop_running,
            iteration_number: iteration_number,
        }
    }

    pub fn start_loop(&mut self) {
        let loop_running = Arc::clone(&self.loop_running);
        let iteration_number = Arc::clone(&self.iteration_number);
        let cameras = Arc::clone(&self.cameras);
        let wfs = Arc::clone(&self.wfs);
        let controller_mut = Arc::clone(&self.controller);
        let dms_mut = Arc::clone(&self.dms);

        loop_running.store(true, std::sync::atomic::Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move || {
            while loop_running.load(std::sync::atomic::Ordering::Relaxed) {
                trace!("Iteration: {}", iteration_number.load(Ordering::Relaxed));

                // Get detector images
                let detector_images = cameras.iter().map(|cam| cam.get_frame()).collect::<Vec<_>>();

                let measurements = wfs.iter().map(|
                        wfs| wfs.measure(&detector_images[wfs.detector_id])
                    ).collect::<Vec<_>>();

                // Compute Commands
                let mut controller = controller_mut.lock().unwrap();
                let commands = controller.compute_commands(&measurements[0]);

                // Apply Commands
                // TODO - sort specific commands to specific DMs
                let mut dms = dms_mut.lock().unwrap();
                dms.iter_mut().for_each(|dm| dm.set_actuators(&commands));

                let iteration = iteration_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                trace!("Iteration: {}", iteration);
            }
        }));
    }

    pub fn stop_loop(&mut self) {
        self.loop_running.store(false,Ordering::Relaxed);
        self.thread_handle.take().map(|h| h.join().unwrap());
    }

    pub fn get_iteration_number(&self) -> u64 {
        self.iteration_number.load(std::sync::atomic::Ordering::Relaxed)
    }
}