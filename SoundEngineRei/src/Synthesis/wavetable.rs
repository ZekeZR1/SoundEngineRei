use core::time::Duration;
use rodio::{OutputStream, source::Source};

pub struct WaveTableSynth{
    wave_table_size: usize
}

impl WaveTableSynth{
    pub fn new(wave_table_size: usize) -> WaveTableSynth{
        return WaveTableSynth{
            wave_table_size: wave_table_size
        }
    }
    pub fn play(&self, frequency: f32, duration: u64){
        let wave_table = self.create_wave_table();
        let sample_rate = 44100;
        let mut oscillator = Oscillator::new(sample_rate, wave_table);
        oscillator.set_frequency(frequency);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(oscillator.convert_samples());

        std::thread::sleep(std::time::Duration::from_secs(duration))
    }

    fn create_wave_table(&self) -> Vec<f32>{
        let mut wave_table:Vec<f32> = Vec::with_capacity(self.wave_table_size);
        for n in 0..self.wave_table_size{
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / self.wave_table_size as f32).sin());
        }
        return wave_table;
    }
}

struct Oscillator{
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32
}

impl Oscillator{
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> Oscillator{
        return Oscillator {
            sample_rate:sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0
        }
    }

    fn set_frequency(&mut self, frequency: f32){
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32{
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32{
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }    
}

impl Source for Oscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for Oscillator{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item>{
        return Some(self.get_sample());
    }
}