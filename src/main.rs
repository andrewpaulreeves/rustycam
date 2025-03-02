use std::thread;
use std::time;
use ndarray::array;
// extern crate intel_mkl_src;
// extern crate blas_src;
use simple_logger::SimpleLogger;

mod fakecamera;
use fakecamera::Camera;

mod fakedm;
use fakedm::DM;

mod wfs;
use wfs::{WFS, ShackHartmann};

mod controller;
use controller::IntegratorController;

mod aoloop;
use aoloop::AOLoop;

// mod centreofgravity;
use wfs::centreofgravity::{simple_centre_of_gravity, threshold_centre_of_gravity, test_cog};


fn test_camera() {
    println!("Hello, Camera!");
    println!("Init Camera...");

    let rows = 128;
    let cols = 128;
    let frame_rate = 0.0 as f32;
    let e_read_noise = 10 as f32;


    let mut cam = Camera::new(rows, cols, e_read_noise, frame_rate);
    println!("Init Camera...Done");
    
    cam.start_acquisition();

    println!("Started Acquisition...");
    println!("Wait 5 seconds...");

    for i in 0..5 {
        thread::sleep(time::Duration::from_secs(1));
        let fr = cam.get_frame_number();
        println!("Frame Number:         {}", fr);
        println!("Frames Per Second:    {}", fr as f32 / (i as f32 + 1.0));

        let frame_buf = cam.get_frame();
        println!("Frame Buf {}", frame_buf);
    }
    println!("Done!");

    cam.stop_acquisition();
    let fr = cam.get_frame_number();
    println!("Frames Per Second: {}", fr as f32 / 5.0);

}

fn test_dm() {
    println!("Hello, DM!");
    println!("Init DM...");

    let n_acts = 140;
    let mut dm = DM::new(n_acts);
    let mut actuator_values = ndarray::Array1::<f32>::zeros((n_acts));
    let mut n: f32 = 0.0;
    for i in actuator_values.iter_mut() {
        *i = n;
        n = n + 1.0;
    }

    for i in 0..n_acts {
        println!("Actuator Value: {}", actuator_values[[i]]);
    }

    dm.set_actuators(&actuator_values);

    println!("DM Acts: {}", dm.get_actuators());

    println!("Done!");
}


fn  test_aoloop() {
    println!("Hello, AO Loop!");
    println!("Init AO Loop...");
    let n_rows = 80;
    let n_cols = 80;
    let frame_rate = 0.0 as f32;
    let e_read_noise = 10 as f32;
    let pixels_per_subap = 8;
    let nx_subaps = (n_rows / pixels_per_subap);
    let n_subaps = nx_subaps * nx_subaps;
    let n_actuators = 140;

    println!("n_subaps: {}", n_subaps);
    println!("nx_subaps: {}", nx_subaps);
    println!("pixels_per_subap: {}", pixels_per_subap);
    println!("n_actuators: {}", n_actuators);

    let mut subap_coordinates = ndarray::Array2::<usize>::zeros((n_subaps, 4));
    for x in 0..nx_subaps{
        for y in 0..nx_subaps {
            subap_coordinates[[x*nx_subaps + y, 0]] = (x*pixels_per_subap);
            subap_coordinates[[x*nx_subaps + y, 1]] = ((x+1)*pixels_per_subap);
            subap_coordinates[[x*nx_subaps + y, 2]] = (y*pixels_per_subap);
            subap_coordinates[[x*nx_subaps + y, 3]] = ((y+1)*pixels_per_subap);
        }
    }

    let mut cam = Camera::new(n_rows, n_cols, e_read_noise, frame_rate);
    cam.start_acquisition();
    let sh = ShackHartmann::new(
        n_rows, n_cols, pixels_per_subap, subap_coordinates, 0);

    let dm = DM::new(2);
    let controller = IntegratorController::new(2*n_subaps, n_actuators, 1.0);
    let mut aoloop = AOLoop::new(vec![cam], vec![sh], controller, vec![dm]);

    println!("Init AO Loop...Done");

    aoloop.start_loop();

    println!("Started AO Loop...");
    println!("Wait 5 seconds...");

    for i in 0..5 {
        thread::sleep(time::Duration::from_secs(1));
        let fr = aoloop.get_iteration_number();
        println!("Frame Number:         {}", fr);
        println!("Frames Per Second:    {:.2}", fr as f32 / (i as f32 + 1.0));
    }
    println!("Done!");

    aoloop.stop_loop();
    let fr = aoloop.get_iteration_number();
    println!("Frames Per Second: {}", fr as f32 / 5.0);

}


fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("Running Rust AO!");
    // test_camera();
    // test_dm();
    // test_cog();
    // wfs::test_shackhartmann();
    test_aoloop();

}
