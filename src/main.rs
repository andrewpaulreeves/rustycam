use std::thread;
use std::time;
use std::sync::atomic::Ordering;

mod fakecamera;
use fakecamera::Camera;

fn main() {
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

        // let fb_ref = cam.frame_buffer.clone();
        // let frame_buf = fb_ref.lock().unwrap();
        let frame_buf = cam.get_frame();
        println!("Frame Buf {}", frame_buf);


    }
    println!("Done!");

    cam.stop_acquisition();
    let fr = cam.get_frame_number();
    println!("Frames Per Second: {}", fr as f32 / 5.0);

}
