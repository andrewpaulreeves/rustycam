/// Controller for the RUST AO software
/// 
/// Controller is responsible for taking WFS measurements, performing offsets,
/// converting to the control basis and implementing a temporal control law 
/// 

use ndarray::{Array1, Array2};

pub struct IntegratorController {
    n_measurements: usize,
    n_commands: usize,
    gain: f32,
    control_matrix: Array2<f32>,
    actuator_commands: Array1<f32>,
}

impl IntegratorController {
    pub fn new(n_measurements: usize, n_commands: usize, gain: f32) -> Self {
        let control_matrix = Array2::<f32>::zeros((n_commands, n_measurements));
        Self {
            n_measurements: n_measurements,
            n_commands: n_commands,
            gain: gain,
            control_matrix: control_matrix,
            actuator_commands: Array1::<f32>::zeros(n_commands),
        }
    }

    pub fn set_control_matrix(&mut self, control_matrix: Array2<f32>) {
        self.control_matrix = control_matrix;
    }

    pub fn get_control_matrix(&self) -> Array2<f32> {
        self.control_matrix.clone()
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn get_gain(&self) -> f32 {
        return self.gain
    }


    pub fn compute_commands(&mut self, measurements: &Array1<f32>) -> Array1<f32> {
        self.actuator_commands = self.actuator_commands.clone() + self.gain * self.control_matrix.dot(measurements);
        return self.actuator_commands.clone();
    }
}