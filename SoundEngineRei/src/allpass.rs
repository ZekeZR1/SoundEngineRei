pub struct AllPass{
    feedback: f64,
    buffer: Vec<f64>,
    bufferidx: usize
}

impl AllPass{
    pub fn new(buffer_length: usize) -> AllPass{
        return AllPass {
            feedback: 0.5,
            buffer: vec![0.0; buffer_length],
            bufferidx: 0
        }
    }

    pub fn set_feedback(&mut self, value: f64){
        self.feedback = value;
    }

    pub fn process(&mut self, input: f64) -> f64{
        let bufout = self.buffer[self.bufferidx];

        let output = -input + bufout;
        self.buffer[self.bufferidx] = input + (bufout * self.feedback);

        self.bufferidx += 1;
        if self.bufferidx >= self.buffer.len(){
            self.bufferidx = 0;
        }

        output
    }
}