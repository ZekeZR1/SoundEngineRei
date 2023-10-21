// Reverb model tuning values

pub const NUM_COMBS: usize = 8;
pub const NUM_ALLPASSES: usize = 4;
pub const MUTED:f64 = 0.0;
pub const FIXED_GAIN: f64 = 0.015;
pub const SCALE_WET: f64 = 3.0;
pub const SCALE_DRY: f64 = 2.0;
pub const SCALE_DAMP: f64 = 0.4;
pub const SCALE_ROOM: f64 = 0.28;
pub const OFFSET_ROOM: f64 = 0.7;
pub const INITIAL_ROOM: f64 = 0.5;
pub const INITIAL_DAMP: f64 = 0.5;
pub const INITIAL_WET: f64 = 1.0 / SCALE_WET;
pub const INITIAL_DRY: f64 = 0.0;
pub const INITIAL_WIDTH: f64 = 1.0;
pub const INITIAL_MODE: f64 = 0.0;
pub const FREEZE_MODE: f64 = 0.5;
pub const STEREO_SPREAD: usize = 23;

// Assuming 44.1KHz Sample rate
pub const COMB_TUNING_L1: usize = 1116;
pub const COMB_TUNING_R1: usize = COMB_TUNING_L1 + STEREO_SPREAD;
pub const COMB_TUNING_L2: usize = 1188;
pub const COMB_TUNING_R2: usize = COMB_TUNING_L2 + STEREO_SPREAD;
pub const COMB_TUNING_L3: usize = 1227;
pub const COMB_TUNING_R3: usize = COMB_TUNING_L3 + STEREO_SPREAD;
pub const COMB_TUNING_L4: usize = 1356;
pub const COMB_TUNING_R4: usize = COMB_TUNING_L4 + STEREO_SPREAD;
pub const COMB_TUNING_L5: usize = 1422;
pub const COMB_TUNING_R5: usize = COMB_TUNING_L5 + STEREO_SPREAD;
pub const COMB_TUNING_L6: usize = 1491;
pub const COMB_TUNING_R6: usize = COMB_TUNING_L6 + STEREO_SPREAD;
pub const COMB_TUNING_L7: usize = 1557;
pub const COMB_TUNING_R7: usize = COMB_TUNING_L7 + STEREO_SPREAD;
pub const COMB_TUNING_L8: usize = 1617;
pub const COMB_TUNING_R8: usize = COMB_TUNING_L8 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L1: usize = 556;
pub const ALLPASS_TUNING_R1: usize = ALLPASS_TUNING_L1 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L2: usize = 441;
pub const ALLPASS_TUNING_R2: usize = ALLPASS_TUNING_L2 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L3: usize = 341;
pub const ALLPASS_TUNING_R3: usize = ALLPASS_TUNING_L3 + STEREO_SPREAD;
pub const ALLPASS_TUNING_L4: usize = 225;
pub const ALLPASS_TUNING_R4: usize = ALLPASS_TUNING_L4 + STEREO_SPREAD;