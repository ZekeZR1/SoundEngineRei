pub struct WaveTableSynth {
    wave_table_size: usize,
}

impl WaveTableSynth {
    pub fn new(wave_table_size: usize) -> WaveTableSynth {
        return WaveTableSynth {
            wave_table_size: wave_table_size,
        };
    }

    pub fn play(&self, frequency: f32, duration: u64) {

    }

    fn create_wave_table(&self) -> Vec<f32> {
        let mut wave_table: Vec<f32> = Vec::with_capacity(self.wave_table_size);
        for n in 0..self.wave_table_size {
            wave_table
                .push((2.0 * std::f32::consts::PI * n as f32 / self.wave_table_size as f32).sin());
        }
        return wave_table;
    }
}