// freeverb
// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html

mod comb;
use comb::Comb;


const NUM_COMBS: usize = 8;
const NUM_ALLPASSES: usize = 4;

pub struct Freeverb{
    gain: f32,
    combs: [(Comb, Comb); NUM_COMBS],
    allpasses: [(AllPass, AllPass); NUM_ALLPASSES],
    wet_gains: (f64, f64),
    wet: f32,
    dry: f32,
}

impl Freeverb{
    pub fn new(gain: f32) -> Freeverb{
        return Freeverb{
            gain: gain,
            combs: [
                (
                    Comb::new(),
                    Comb::new()
                ),
                // x8
            ],
            allpasses: [
                (
                    AllPass::new(),
                    AllPass::new()
                )
                // x4
            ],
            wet_gains: (0,0),
        }
    }

    fn process(&mut self, input: (f64, f64)) -> (f64, f64){
        let mut out = (0 as f64, 0 as f64);
        let input_mixed = (input.0 + input.1) * self.gain;

        // comb filters
        for combs in self.combs.iter_mut(){
            out.0 += combs.0.process(input_mixed);
            out.1 += combs.1.process(input_mixed);
        }

        // all passes
        for allpasses in self.allpasses.iter_mut(){
            out.0
        }

        // output
        out.0 = out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry;
        out.1 = out.1 * self.wet_gains.1 + out.0 * self.wet_gains.0 + input.1 * self.dry;

        return out;
    }
}

pub struct AllPass{

}

impl AllPass{
    pub fn new() -> AllPass{
        return AllPass{

        }
    }

    pub fn process(&mut self, input: f64) -> f64{
        // todo all pass filter
        
        return input;
    }
}