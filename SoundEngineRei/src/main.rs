use Synthesis::wavetable::WaveTableSynth;
mod Synthesis;

fn main(){
    let synth = WaveTableSynth::new(32);
    synth.play(440.0, 2);   
}