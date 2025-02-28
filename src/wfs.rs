/// RUST-AO Wavefront Sensing
/// 
/// This module contains  algorithms that operate on detector
/// frame, and output vectors representin wavefront measurements.
/// The WFS also is responsible for calibrating detector pixels
/// 

use ndarray::{Array1, Array2, Array3, s};

pub mod centreofgravity;
use centreofgravity::{simple_centre_of_gravity, threshold_centre_of_gravity};

/// Constructs a new Wavefront Sensor object;
pub struct WFS {
    n_rows: usize,
    n_cols: usize,
    n_measurements: usize,
} 

pub struct ShackHartmann {
    n_rows: usize,
    n_cols: usize,
    n_measurements: usize,
    n_subaps: usize,
    pixels_per_subap: usize,
    subap_coordinates: Array2<usize>,
    dark_subaps: Array3<f32>,
    flat_subaps: Array3<f32>,
    bg_subaps: Array3<f32>,
    cal_subaps: Array3<f32>
}

impl ShackHartmann {
    pub fn new(
            n_rows: usize, n_cols: usize, 
            pixels_per_subap:usize, subap_coordinates: Array2<usize>) -> Self {

        let n_subaps = subap_coordinates.shape()[0];
        let n_measurements = 2 * n_subaps;

        let cal_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let dark_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let bg_subaps = Array3::<f32>::zeros((n_subaps, pixels_per_subap, pixels_per_subap));
        let flat_subaps = Array3::<f32>::ones((n_subaps, pixels_per_subap, pixels_per_subap));

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
            flat_subaps: flat_subaps
        }
    }

    pub fn measure(&self, frame: &Array2<u16>) -> Array1<f32> {
        let mut measurements = Array1::<f32>::zeros(self.n_measurements);

        for i in 0..self.n_subaps {
            // Get the coordinates of this particular sub-aperture
            let subap_coords = &self.subap_coordinates.slice(s![i, ..]);
            
            // Slice out the data of that Sub-aperture (maybe want to actually copy the data out for processing)
            let subap_data = frame.slice(s![subap_coords[0]..subap_coords[1], subap_coords[2]..subap_coords[3]]).to_owned();

            let mut cal_subap = self.cal_subaps.slice(s![i, .., ..]).to_owned();
            let dark_subap = self.dark_subaps.slice(s![i, .., ..]);
            let bg_subap = self.bg_subaps.slice(s![i, .., ..]);
            let flat_subap = self.flat_subaps.slice(s![i, .., ..]);

            // Pixel calibration
            for x in 0..self.pixels_per_subap {
                for y in 0..self.pixels_per_subap {
                    cal_subap[[x, y]] = (subap_data[[x, y]] as f32 - bg_subap[[x, y]] - dark_subap[[x, y]]) / flat_subap[[x, y]];
                }
            }

            // CoG computation
            let subap_cal = self.cal_subaps.slice(s![i, .., ..]).to_owned();
            let (x, y) = simple_centre_of_gravity(&subap_cal);

            // Update the measurements vector
            measurements[i] = x;
            measurements[i + self.n_subaps] = y;
        }
        measurements
    }
}


