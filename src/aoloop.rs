
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::option;
use std::time::{Duration, Instant};
use log::{trace, debug, info, warn};

use crate::fakecamera::Camera;
use crate::wfs::ShackHartmann;
use crate::fakedm::DM;
use crate::controller::IntegratorController;
use crate::shmupdater::ShmUpdater;

pub struct AOLoop {
    cameras: Arc<Vec<Camera>>,
    wfs: Arc<Vec<ShackHartmann>>,
    controller: Arc<Mutex<IntegratorController>>,
    dms: Arc<Mutex<Vec<DM>>>,
    thread_handle: option::Option<thread::JoinHandle<()>>,
    loop_running: Arc<AtomicBool>,
    iteration_number: Arc<AtomicU64>,
    timer: Arc<Mutex<LoopTimers>>,
    shm_updater: Arc<Mutex<ShmUpdater>>,
}


struct LoopTimers {
    pub total_time: Duration,
    pub cam_time:   Duration,
    pub wfs_time:   Duration,
    pub ctrl_time:  Duration,
    pub dm_time:    Duration,
}

impl LoopTimers{
    pub fn print(&self) {
        info!("Total Time:      {:?}",       self.total_time);
        info!("Camera Time:     {:?}",      self.cam_time);
        info!("WFS Time:        {:?}", self.wfs_time);
        info!("Controller Time: {:?}", self.ctrl_time);
        info!("DM Time:         {:?}", self.dm_time);
    }
}

impl AOLoop {
    pub fn new(cameras: Vec<Camera>, wfs: Vec<ShackHartmann>, controller: IntegratorController, dms: Vec<DM>) -> Self {
        let loop_running = Arc::new(AtomicBool::new(false));
        let iteration_number = Arc::new(AtomicU64::new(0));

        let timer = LoopTimers{
            total_time: Duration::new(0, 0),
            cam_time: Duration::new(0, 0),
            wfs_time: Duration::new(0, 0),
            ctrl_time: Duration::new(0, 0),
            dm_time: Duration::new(0, 0),
        };

        let shm_updater = ShmUpdater::new(
            wfs[0].n_measurements, dms[0].n_acts, cameras[0].n_rows, cameras[0].n_cols
        );
        Self {
            cameras: Arc::new(cameras),
            wfs: Arc::new(wfs),
            controller: Arc::new(Mutex::new(controller)),
            dms: Arc::new(Mutex::new(dms)),
            thread_handle: None,
            loop_running: loop_running,
            iteration_number: iteration_number,
            timer: Arc::new(Mutex::new(timer)),
            shm_updater: Arc::new(Mutex::new(shm_updater)),
        }
    }

    pub fn start_loop(&mut self) {
        let loop_running = Arc::clone(&self.loop_running);
        let iteration_number = Arc::clone(&self.iteration_number);
        let cameras = Arc::clone(&self.cameras);
        let wfs = Arc::clone(&self.wfs);
        let controller_mut = Arc::clone(&self.controller);
        let dms_mut = Arc::clone(&self.dms);
        let timer_mutex = Arc::clone(&self.timer);
        let shm_updater_mutex = self.shm_updater.clone();

        loop_running.store(true, std::sync::atomic::Ordering::Relaxed);

        self.thread_handle = option::Option::Some(std::thread::spawn(move || {
            while loop_running.load(std::sync::atomic::Ordering::Relaxed) {
                let loop_start = Instant::now();
                trace!("Iteration: {}", iteration_number.load(Ordering::Relaxed));

                let mut timer = timer_mutex.lock().unwrap();

                // Get detector images
                let cam_start = Instant::now();
                let detector_images = cameras.iter().map(|cam| cam.get_frame()).collect::<Vec<_>>();
                timer.cam_time += cam_start.elapsed();

                let wfs_start = Instant::now();
                let measurements = wfs.iter().map(|
                        wfs| wfs.measure(&detector_images[wfs.detector_id])
                    ).collect::<Vec<_>>();
                timer.wfs_time += wfs_start.elapsed();

                // Compute Commands
                let ctrl_start = Instant::now();
                let mut controller = controller_mut.lock().unwrap();
                let commands = controller.compute_commands(&measurements[0]);
                timer.ctrl_time += ctrl_start.elapsed();

                // Apply Commands
                // TODO - sort specific commands to specific DMs
                let dm_start = Instant::now();
                let mut dms = dms_mut.lock().unwrap();
                dms.iter_mut().for_each(|dm| dm.set_actuators(&commands));
                timer.dm_time += dm_start.elapsed();

                let iteration = iteration_number.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                trace!("Iteration: {}", iteration);

                let mut shm_updater = shm_updater_mutex.lock().unwrap();
                // shm_updater.update_camera_frame(&detector_images[0], iteration);
                shm_updater.update_actuator_commands(&commands, iteration);
                shm_updater.update_wfs_measurements(&measurements[0], iteration);

                timer.total_time += loop_start.elapsed();
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

    pub fn print_timers(&self) {
        let timer = self.timer.lock().unwrap();
        timer.print();

        let iteration_number = self.iteration_number.load(Ordering::Relaxed);
        info!("\nPer Iteration:");
        info!("Iteration Time:      {:?} ns", timer.total_time.as_nanos() / iteration_number as u128);
        info!("Camera Time:         {:?} ns", timer.cam_time.as_nanos() / iteration_number as u128);
        info!("WFS Time:            {:?} ns", timer.wfs_time.as_nanos() / iteration_number as u128);
        info!("Controller Time:     {:?} ns", timer.ctrl_time.as_nanos() / iteration_number as u128);
        info!("DM Time:             {:?} ns", timer.dm_time.as_nanos() / iteration_number as u128);
    }
}
