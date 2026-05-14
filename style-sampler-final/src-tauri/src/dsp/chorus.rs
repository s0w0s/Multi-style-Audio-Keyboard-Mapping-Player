pub struct Chorus {
    sample_rate: f32,
    mix: f32,
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    lfo_freq: f32,
    depth: f32,
    delay_base: usize,
}

impl Chorus {
    pub fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 2.0) as usize;
        Self {
            sample_rate,
            mix: 0.0,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_freq: 0.5,
            depth: 0.002,
            delay_base: (sample_rate * 0.01) as usize,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.mix <= 0.001 {
            return input;
        }

        self.lfo_phase += self.lfo_freq / self.sample_rate;
        if self.lfo_phase > 1.0 {
            self.lfo_phase -= 1.0;
        }
        
        let lfo_value = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_samples = self.delay_base + (lfo_value * self.depth * self.sample_rate) as usize;
        
        let buffer_len = self.buffer.len();
        let clamped_delay = delay_samples.min(buffer_len - 1).max(1);
        
        let read_pos = if self.write_pos >= clamped_delay {
            self.write_pos - clamped_delay
        } else {
            buffer_len - (clamped_delay - self.write_pos)
        };
        
        let delayed = self.buffer[read_pos];
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % buffer_len;
        
        input * (1.0 - self.mix) + delayed * self.mix
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_freq = rate.clamp(0.1, 5.0);
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.001, 0.01);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.delay_base = (sample_rate * 0.01) as usize;
    }
}

impl Default for Chorus {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
