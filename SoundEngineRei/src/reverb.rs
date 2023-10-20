// freeverb
// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html

use crate::{allpass::AllPass, comb::Comb};
use crate::tuning;

pub struct Freeverb{
    gain: f64,
    room_size: (f64, f64),
    wet: f64,
    wet_gains: (f64, f64),
    dry: f64,
    width: f64,
    mode: f64,
    combs: [(Comb, Comb); tuning::NUM_COMBS],
    allpasses: [(AllPass, AllPass); tuning::NUM_ALLPASSES],
}

fn adjust_length(length: usize, sample_rate: usize) -> usize{
    (length as f64 * sample_rate as f64 / 44100.0) as usize
}

impl Freeverb{
    pub fn new(sample_rate: usize) -> Freeverb{
        let mut freeverb = Freeverb{
            gain: tuning::FIXED_GAIN,
            room_size: (0.0, 0.0),
            wet: 0.0,
            wet_gains: (0.0,0.0),
            dry: 0.0,
            width: 0.0,
            mode: 0.0,
            combs: [
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L1, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R1, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L2, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R2, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L3, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R3, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L4, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R4, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L5, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R5, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L6, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R6, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L7, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R7, sample_rate)),              
                ),
                (
                    Comb::new(adjust_length(tuning::COMB_TUNING_L8, sample_rate)),
                    Comb::new(adjust_length(tuning::COMB_TUNING_R8, sample_rate)),              
                ),
            ],
            allpasses: [
                (
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_L1, sample_rate)),
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_R1, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_L2, sample_rate)),
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_R2, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_L3, sample_rate)),
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_R3, sample_rate)),
                ),
                (
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_L4, sample_rate)),
                    AllPass::new(adjust_length(tuning::ALLPASS_TUNING_R4, sample_rate)),
                ),
            ]
        };

        // todo 
        //freeverb.set

        freeverb
    }

    fn update(&mut self){
        // todo 
    }

    fn set_wet(&mut self, value: f64){
        self.wet = value * tuning::SCALE_WET;
        self.update();
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