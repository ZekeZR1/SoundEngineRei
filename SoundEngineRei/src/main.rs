mod wavetable;
use wavetable::WaveTableSynth;
mod reverb;
mod comb;
mod allpass;
mod tuning;

fn main(){
    let synth = WaveTableSynth::new(32);
    synth.play(440.0, 2);   
}