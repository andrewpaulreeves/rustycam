use std::thread;
use std::time;
use std::sync::atomic::Ordering;

use simple_logger::SimpleLogger;

mod fakecamera;
use fakecamera::Camera;

mod fakedm;
use fakedm::DM;

mod wfs;
use wfs::WFS;

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
    let mut actuator_values = ndarray::Array2::<f32>::zeros((n_acts, 1));
    let mut n: f32 = 0.0;
    for i in actuator_values.iter_mut() {
        *i = n;
        n = n + 1.0;
    }

    for i in 0..n_acts {
        println!("Actuator Value: {}", actuator_values[[i, 0]]);
    }

    dm.set_actuators(&actuator_values);

    println!("DM Acts: {}", dm.get_actuators());

    println!("Done!");
}



fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Running Rust AO!");
    // test_camera();
    // test_dm();
    // test_cog();
    wfs::test_shackhartmann();

}
