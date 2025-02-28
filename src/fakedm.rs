use ndarray::Array1;

pub struct DM {
    n_acts: usize,
    act_buffer: Array1<f32>,
}

impl DM {
    pub fn new(n_acts: usize) -> Self {
        let act_shape = (n_acts,);
        let act_buffer = Array1::<f32>::zeros(act_shape);
        Self{
            n_acts: n_acts,
            act_buffer: act_buffer,
        }
    }

    pub fn set_actuators(&mut self, actuator_values: &Array1<f32>) {
        actuator_values.clone_into(&mut self.act_buffer);
    }

    pub fn get_actuators(&self) -> Array1<f32> {
        self.act_buffer.clone()
    }
}
