use hound;
use std::str;

pub struct WavReader {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: f32,
    pub duration: u32
}

impl WavReader {
    pub fn new() -> WavReader {
        WavReader {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16.0,
            duration: 0
        }
    }

    pub fn read_wave_file_samples(&mut self, file_path: &str) -> Vec<f32> {
        let mut buffer: Vec<f32> = vec![];
        let mut reader = hound::WavReader::open(file_path).unwrap();
        let spec = reader.spec();
        let file_sr = spec.sample_rate;
        let dur = reader.duration() / file_sr;
        let samples = reader.samples::<i16>();

        self.channels = spec.channels;
        self.sample_rate = file_sr;
        self.bits_per_sample = spec.bits_per_sample as f32;
        self.duration = dur;

        for sample in samples {
            buffer.push(sample.unwrap_or(0) as f32 / i16::MAX as f32);
        }

        buffer
    }
}
