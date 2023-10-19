pub struct Comb{
    feedback: f64, 
    filterstore: f64,
    damp1: f64,
    damp2: f64,
    buffer: Vec<f64>,
    bufferidx: usize
}

impl Comb{
    pub fn new(buffer_length: usize) -> Comb{
        return Comb{
            feedback: 0.5,
            filterstore: 0.0,
            damp1: 0.5,
            damp2: 0.5,
            buffer: vec![0.0; buffer_length],
            bufferidx: 0
        }
    }

    pub fn process(&mut self, input: f64) -> f64{
        let output = self.buffer[self.bufferidx];

        self.filterstore = output * self.damp2 + self.filterstore * self.damp1;

        self.buffer[self.bufferidx] = input + (self.filterstore * self.feedback);

        self.bufferidx += 1;
        if self.bufferidx > self.buffer.len() - 1 {
            self.bufferidx = 0
        }

        return output;
    }
}