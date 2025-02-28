use ndarray::Array2;
use std::time::{Instant, SystemTime};

pub fn simple_centre_of_gravity(data: &Array2<f32>) -> (f32, f32) {
    let (n_rows, n_cols) = data.dim();
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut total: f32 = 0.0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            let val = data[[i, j]];
            x += val * i as f32;
            y += val * j as f32;
            total += val;
        }
    }
    (x / total - n_rows as f32 / 2.0 + 0.5, y / total - n_rows as f32 / 2.0 + 0.5)
}

pub fn threshold_centre_of_gravity(data: &Array2<f32>, threshold: f32) -> (f32, f32) {
    let (n_rows, n_cols) = data.dim();
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut total: f32 = 0.0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            let val = data[[i, j]];
            if val > threshold {
                x += val * i as f32;
                y += val * j as f32;
                total += val;
            }
        }
    }
    (x / total - n_rows as f32 / 2.0 + 0.5, y / total - n_rows as f32 / 2.0 + 0.5)
}

fn bench_simple_centre_of_gravity() {

    let n = 1e6 as u32;    
    let data = Array2::<f32>::ones((8, 8));
    let t = Instant::now();
    for _ in 0..n {
        let (x, y) = simple_centre_of_gravity(&data);
    }
    println!("Nx = 8: Simple COG: {} ns", 1e9 * t.elapsed().as_secs_f32() / n as f32);

    let n = 1e5 as u32;    
    let data = Array2::<f32>::ones((16, 16));
    let t = Instant::now();
    for _ in 0..n {
        let (x, y) = simple_centre_of_gravity(&data);
    }
    println!("Nx = 16: Simple COG: {} ns", 1e9 * t.elapsed().as_secs_f32() / n as f32);

    let n = 1e5 as u32;    
    let data = Array2::<f32>::ones((32, 32));
    let t = Instant::now();
    for _ in 0..n {
        let (x, y) = simple_centre_of_gravity(&data);
    }
    println!("Nx = 32: Simple COG: {} ns", 1e9 * t.elapsed().as_secs_f32() / n as f32);

}
pub fn test_cog() {
    let data = Array2::<f32>::ones((10, 10));
    let (x, y) = simple_centre_of_gravity(&data);
    println!("Simple COG: ({}, {})", x, y);
    let (x, y) = threshold_centre_of_gravity(&data, 0.);
    println!("Threshold COG: ({}, {})", x, y);

    bench_simple_centre_of_gravity();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_centre_of_gravity_zero() {
        let data = Array2::<f32>::ones((8, 8));
        let (x, y) = simple_centre_of_gravity(&data);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
    #[test]
    fn test_threshold_centre_of_gravity_zero() {
        let data = Array2::<f32>::ones((8, 8));
        let (x, y) = threshold_centre_of_gravity(&data, 0.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
}

