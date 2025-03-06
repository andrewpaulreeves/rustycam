/// RUST-AO Wavefront Sensing
/// 
/// This module contains  algorithms that operate on detector
/// frame, and output vectors representin wavefront measurements.
/// The WFS also is responsible for calibrating detector pixels
/// 
use log::{trace, debug, info, warn};
use ndarray::{Array, array, Array1, Array2, Array3, s};
use std::sync::{Arc, Mutex};

pub mod centreofgravity;
use centreofgravity::{simple_centre_of_gravity, threshold_centre_of_gravity};
use rayon::prelude::*;

/// Constructs a new Wavefront Sensor object;
pub struct WFS {
    n_rows: usize,
    n_cols: usize,
    n_measurements: usize,
} 
#[allow(dead_code)]
pub struct ShackHartmann {
    n_rows: usize,
    n_cols: usize,
    pub n_measurements: usize,
    n_subaps: usize,
    pixels_per_subap: usize,
    subap_coordinates: Vec<Vec<usize>>,
    dark_subaps: Array3<f32>,
    flat_subaps: Array3<f32>,
    bg_subaps: Array3<f32>,
    cal_subaps: Array3<f32>,
    pub detector_id: usize,
    measurements: Arc<Mutex<Array1<f32>>>,
}

impl ShackHartmann {
    pub fn new(
            n_rows: usize, n_cols: usize, 
            pixels_per_subap:usize, subap_coordinates: Vec<Vec<usize>>,
            detector_id: usize) -> Self {


        let n_subaps = subap_coordinates.len();
        let n_measurements = 2 * n_subaps;

        let cal_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let dark_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let bg_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let flat_subaps = Array3::<f32>::ones((n_subaps, pixels_per_subap, pixels_per_subap));

        info!("ShackHartmann: Created new ShackHartmann Sensor");
        info!("n_subaps: {}", n_subaps);
        info!("n_measurements: {}", n_measurements);

        let measurements = Arc::new(Mutex::new(Array1::<f32>::zeros(n_measurements)));

        Self {
            n_rows: n_rows,
            n_cols: n_cols,
            n_measurements: n_measurements,
            n_subaps: n_subaps,
            pixels_per_subap: pixels_per_subap,
            subap_coordinates: subap_coordinates,
            cal_subaps: cal_subaps,
            dark_subaps: dark_subaps,
            bg_subaps: bg_subaps,
            flat_subaps: flat_subaps,
            detector_id: detector_id,
            measurements: measurements
        }
    }

    pub fn measure(&self, frame: &Array2<u16>) -> Array1<f32> {

        let measurements_mutex = &self.measurements;

        self.subap_coordinates.par_iter().enumerate().for_each(|(i, subap_coords)| {

            // Slice out the data of that Sub-aperture (maybe want to actually copy the data out for processing)
            let subap_data = frame.slice(
                s![
                    subap_coords[0]..subap_coords[1],
                    subap_coords[2]..subap_coords[3]
                    ]).to_owned();

            let mut cal_subap = self.cal_subaps.slice(s![i, .., ..]).to_owned();
            let dark_subap = self.dark_subaps.slice(s![i, .., ..]);
            let bg_subap = self.bg_subaps.slice(s![i, .., ..]);
            let flat_subap = self.flat_subaps.slice(s![i, .., ..]);

            // Pixel calibration
            cal_subap.iter_mut().enumerate().for_each(|(idx, pixel)| {
                let x = idx / self.pixels_per_subap;
                let y = idx % self.pixels_per_subap;
                *pixel = (subap_data[[x, y]] as f32 - bg_subap[[x, y]] - dark_subap[[x, y]]) / flat_subap[[x, y]];
            });

            // CoG computation
            let (x, y) = simple_centre_of_gravity(&cal_subap);
 
            // Update the measurements vector
            let mut measurements = measurements_mutex.lock().unwrap();
            measurements[i] = x;
            measurements[i + self.n_subaps] = y;


            trace!("subap: {}", i);
            trace!("subap_coords: x: {}-{}, y: {}-{}", subap_coords[0], subap_coords[1], subap_coords[2], subap_coords[3]);
            trace!("subap_data:\n{:?}", subap_data);
            trace!("cal_subap:\n{:?}", cal_subap);
            trace!("x: {}, y: {}", x, y);
        });
        measurements_mutex.lock().unwrap().to_owned()
    }


}


pub fn test_shackhartmann () {
    let n_rows = 16;
    let n_cols = 16;
    let pixels_per_subap = 8;
    let subap_coordinates = vec![
        vec![0, 8, 0, 8], 
        vec![0, 8, 8, 16], 
        vec![8, 16, 0, 8], 
        vec![8, 16, 8, 16]
    ];


    let sh = ShackHartmann::new(n_rows, n_cols, pixels_per_subap, subap_coordinates, 0);

    let frame = Array::from_iter(0..(n_rows*n_cols) as u16).to_shape((n_rows, n_cols)).unwrap().to_owned();


    let measurements = sh.measure(&frame);
    println!("{:?}", measurements);
    let expected = array![1.4117646, 1.2444444, 0.44799995, 0.42966747, 0.08823538, 0.07777786, 0.028000116, 0.026854277];
    assert_eq!(measurements, expected);
}