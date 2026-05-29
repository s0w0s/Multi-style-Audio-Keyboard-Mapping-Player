pub struct Delay {
    sample_rate: f32,
    time: f32,
    feedback: f32,
    mix: f32,
    buffer: Vec<f32>,
    write_pos: usize,
    delay_samples: usize,
}

impl Delay {
    pub fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 2.0) as usize;
        Self {
            sample_rate,
            time: 0.3,
            feedback: 0.4,
            mix: 0.5,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            delay_samples: (sample_rate * 0.3) as usize,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.time <= 0.001 {
            return input;
        }

        self.delay_samples = (self.sample_rate * self.time) as usize;
        
        let delay_samples = self.delay_samples.min(self.buffer.len() - 1);
        let read_pos = if self.write_pos >= delay_samples {
            self.write_pos - delay_samples
        } else {
            self.buffer.len() - (delay_samples - self.write_pos)
        };
        
        let delayed = self.buffer[read_pos];
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        
        input * (1.0 - self.mix) + delayed * self.mix
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_time(&mut self, time: f32) {
        self.time = time.clamp(0.0, 2.0);
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.95);
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        let buffer_size = (sample_rate * 2.0) as usize;
        self.buffer = vec![0.0; buffer_size];
        self.write_pos = 0;
        self.delay_samples = (sample_rate * self.time) as usize;
    }
}
