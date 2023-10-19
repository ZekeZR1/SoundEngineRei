mod wavetable;
use wavetable::WaveTableSynth;
//mod reverb;
mod comb;
fn main(){
    let synth = WaveTableSynth::new(32);
    synth.play(440.0, 2);   
}