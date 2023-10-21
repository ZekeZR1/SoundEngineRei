mod Oscillator;
mod allpass;
mod comb;
mod reverb;
mod tuning;
mod wavetable;
mod read;
use crate::reverb::Freeverb;
use rodio::buffer::SamplesBuffer;
use rodio::{source::Source, OutputStream};
use read::WavReader;

const SAMPLE_RATE: u32 = 44100;

fn main() {
    let path = "sound/drums1.wav";

    let mut wav_reader = WavReader::new();
    let samples = wav_reader.read_wave_file_samples(path);

    // FX
    let mut reverb = Freeverb::new(SAMPLE_RATE as usize);
    reverb.set_wet(0.5);
    reverb.set_dry(0.5);
    reverb.set_room_size(0.98);

    let mut buf_original: Vec<f32> = vec![];
    let mut buf_reverb: Vec<f32> = vec![];
    for sample in samples {
        let filtered = reverb.process((sample as f64, sample as f64));
        let mixed = filtered.1 + filtered.0;
        buf_original.push(sample * 2.0);
        buf_reverb.push(mixed as f32);
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // original
    let buffer = SamplesBuffer::new(wav_reader.channels, wav_reader.sample_rate, buf_original);
    let _result = stream_handle.play_raw(buffer.convert_samples());

    let offset_sec = 3;
    let sleep_sec = (wav_reader.duration + offset_sec) as u64;
    std::thread::sleep(std::time::Duration::from_secs(sleep_sec));
    
    // reverb
    let buffer = SamplesBuffer::new(wav_reader.channels, wav_reader.sample_rate, buf_reverb);
    let _result = stream_handle.play_raw(buffer.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(sleep_sec));
}
