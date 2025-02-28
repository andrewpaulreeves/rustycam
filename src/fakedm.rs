use ndarray::Array2;

pub struct DM {
    n_acts: usize,
    act_buffer: Array2<f32>,
}

impl DM {
    pub fn new(n_acts: usize) -> Self {
        let act_shape = (n_acts, 1);
        let act_buffer = Array2::<f32>::zeros(act_shape);
        Self{
            n_acts: n_acts,
            act_buffer: act_buffer,
        }
    }

    pub fn set_actuators(&mut self, actuator_values: &Array2<f32>) {
        actuator_values.clone_into(&mut self.act_buffer);
    }

    pub fn get_actuators(&self) -> Array2<f32> {
        self.act_buffer.clone()
    }
}
