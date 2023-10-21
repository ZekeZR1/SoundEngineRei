// freeverb
// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html

use crate::{allpass::AllPass, comb::Comb};
use crate::{tuning, allpass};

pub struct Freeverb{
    gain: f64,
    room_size: f64,
    damp: f64,
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
            room_size: 0.0,
            damp: 0.0,
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

        freeverb.set_wet(tuning::INITIAL_WET);
        freeverb.set_room_size(tuning::INITIAL_ROOM);
        freeverb.set_dry(tuning::INITIAL_DRY);
        freeverb.set_damp(tuning::INITIAL_DAMP);
        freeverb.set_width(tuning::INITIAL_WIDTH);
        freeverb.set_mode(tuning::INITIAL_MODE);

        freeverb
    }

    pub fn process(&mut self, input: (f64, f64)) -> (f64, f64){
        let input_mixed = (input.0 + input.1) * self.gain;

        let mut output = (0.0, 0.0);
 
        for combs in self.combs.iter_mut(){
            output.0 += combs.0.process(input_mixed);
            output.1 += combs.1.process(input_mixed);
        };

        for allpasses in self.allpasses.iter_mut(){
            output.0 = allpasses.0.process(output.0);
            output.1= allpasses.1.process(output.1);
        };

        (
            output.0 * self.wet_gains.0 + output.1 * self.wet_gains.1 + input.0 * self.dry,
            output.1 * self.wet_gains.0 * output.0 * self.wet_gains.1 * input.1 * self.dry
        )
    }

    pub fn set_room_size(&mut self, value: f64){
        self.room_size = value * tuning::SCALE_ROOM + tuning::OFFSET_ROOM;
        self.update();
    }

    pub fn set_damp(&mut self, value: f64){
        self.damp = value * tuning::SCALE_DAMP;
        self.update();
    }

    pub fn set_wet(&mut self, value: f64){
        self.wet = value * tuning::SCALE_WET;
        self.update();
    }

    pub fn set_dry(&mut self, value: f64){
        self.dry = value * tuning::SCALE_DRY;
    }

    pub fn set_width(&mut self, value: f64){
        self.width = value;
        self.update();
    }

    pub fn set_mode(&mut self, value: f64){
        self.mode = value;
        self.update();
    }
    
    fn update(&mut self){
        // wet
        self.wet_gains = (
            self.wet * (self.width / 2.0 * 0.5),
            self.wet * ((1.0 - self.width) / 2.0)
        );

        // combs
        let mut room_size = self.room_size;
        let mut damp = self.damp;
        let mut gain = tuning::FIXED_GAIN;
        if self.mode >= tuning::FREEZE_MODE{
            room_size = 1.0;
            damp = 0.0;
            gain = tuning::MUTED;
        }
        for combs in self.combs.iter_mut(){
            // fb
            combs.0.set_feedback(room_size);
            combs.1.set_feedback(room_size);
            // damp
            combs.0.set_damp(damp);
            combs.1.set_damp(damp);
        }
    }

}