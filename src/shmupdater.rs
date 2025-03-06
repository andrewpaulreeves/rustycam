use std::vec;
use ndarray::{Array1, Array2};
use aosharedmemory::shmwriter::AoShmWriter;
use aosharedmemory::shmcommon::AO_DTYPE;

pub struct ShmUpdater {
    wfs_measurements_shm_writer: AoShmWriter,
    actuator_shm_writer: AoShmWriter,
    camera_shm_writer: AoShmWriter,
}

impl ShmUpdater {
    pub fn new(n_measurements: usize, n_actuators: usize, cam_n_rows: usize, cam_n_cols: usize) -> Self {
        let fifo_size = 8;
        let wfs_measurements_shm_writer = AoShmWriter::new(
                "wfs_measurements",
                vec![cam_n_rows as u64, cam_n_cols as u64],
                AO_DTYPE::FLOAT32,
                8
            );
        let actuator_shm_writer = AoShmWriter::new(
                "actuator_commands",
                vec![n_actuators as u64],
                AO_DTYPE::FLOAT32,
                8
            );
        let camera_shm_writer = AoShmWriter::new(
                "actuator_commands",
                vec![n_measurements as u64],
                AO_DTYPE::UINT16,
                8
            );

        Self {
            wfs_measurements_shm_writer,
            actuator_shm_writer,
            camera_shm_writer,
        }
    }

    pub fn update_wfs_measurements(&mut self, measurements: Array1<f32>, iter_num: u64) {
        let datau8_vec: Vec<u8> = measurements.iter().flat_map(|&x| x.to_ne_bytes().to_vec()).collect();
        self.wfs_measurements_shm_writer.set_next_frame(datau8_vec, iter_num);
    }

    pub fn update_actuator_commands(&mut self, commands: Array1<f32>, iter_num: u64) {
        let datau8_vec: Vec<u8> = commands.iter().flat_map(|&x| x.to_ne_bytes().to_vec()).collect();
        self.actuator_shm_writer.set_next_frame(datau8_vec, iter_num);
    }

    pub fn update_camera_frame(&mut self, frame: Array2<u16>, iter_num: u64) {
        let datau8_vec: Vec<u8> = frame.iter().flat_map(|&x| x.to_ne_bytes().to_vec()).collect();
        self.camera_shm_writer.set_next_frame(datau8_vec, iter_num);
    }
}