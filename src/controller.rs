/// Controller for the RUST AO software
/// 
/// Controller is responsible for taking WFS measurements, performing offsets,
/// converting to the control basis and implementing a temporal control law 
/// 
pub struct IntegratorController {
    n_measurements: usize,
    n_commands: usize,
    gain: f32,
    control_matrix: Array2<f32>,
}

impl IntegratorController {
    pub fn new(n_measurements: usize, n_commands: usize, gain: f32) -> Self {
        let control_matrix = Array2::<f32>::zeros((n_commands, n_measurements));
        Self {
            n_measurements: n_measurements,
            n_commands: n_commands,
            gain: gain,
            control_matrix: control_matrix,
        }
    }

    pub fn set_control_matrix(&mut self, control_matrix: Array2<f32>) {
        self.control_matrix = control_matrix;
    }

    pub fn get_control_matrix(&self) -> Array2<f32> {
        self.control_matrix.clone()
    }

    pub fn set_gain(&self, gain: f32) {
        self.gain = gain;
    }

    pub fn get_gain(&self) -> f32 {
        self.gain
    }


    pub fn compute_commands(&self, measurements: Array1<f32>) -> Array1<f32> {
        let commands = self.commands + self.gain * self.control_matrix.dot(&measurements);
        commands
    }
}