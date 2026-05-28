pub struct Reverb {
    sample_rate: f32,
    mix: f32,
    comb_buffers: Vec<Vec<f32>>,
    comb_write_pos: Vec<usize>,
    comb_delays: Vec<usize>,
    comb_feedback: Vec<f32>,
    allpass_buffers: Vec<Vec<f32>>,
    allpass_write_pos: Vec<usize>,
    allpass_delays: Vec<usize>,
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        let comb_delays: Vec<usize> = vec![
            (sample_rate * 0.0297) as usize,
            (sample_rate * 0.0371) as usize,
            (sample_rate * 0.0411) as usize,
            (sample_rate * 0.0437) as usize,
        ];
        
        let comb_buffers: Vec<Vec<f32>> = comb_delays
            .iter()
            .map(|d| vec![0.0; *d])
            .collect();
        
        let comb_write_pos = vec![0usize; 4];
        let comb_feedback = vec![0.7f32; 4];
        
        let allpass_delays: Vec<usize> = vec![
            (sample_rate * 0.0197) as usize,
            (sample_rate * 0.0271) as usize,
            (sample_rate * 0.0311) as usize,
            (sample_rate * 0.0337) as usize,
        ];
        
        let allpass_buffers: Vec<Vec<f32>> = allpass_delays
            .iter()
            .map(|d| vec![0.0; *d])
            .collect();
        
        let allpass_write_pos = vec![0usize; 4];

        Self {
            sample_rate,
            mix: 0.0,
            comb_buffers,
            comb_write_pos,
            comb_delays,
            comb_feedback,
            allpass_buffers,
            allpass_write_pos,
            allpass_delays,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.mix <= 0.001 {
            return input;
        }

        let mut output = 0.0f32;
        
        for i in 0..4 {
            let delay_samples = self.comb_delays[i];
            let buffer_len = self.comb_buffers[i].len();
            let read_pos = if self.comb_write_pos[i] >= delay_samples {
                self.comb_write_pos[i] - delay_samples
            } else {
                buffer_len - (delay_samples - self.comb_write_pos[i])
            };
            
            let delayed = self.comb_buffers[i][read_pos];
            self.comb_buffers[i][self.comb_write_pos[i]] = input + delayed * self.comb_feedback[i];
            self.comb_write_pos[i] = (self.comb_write_pos[i] + 1) % buffer_len;
            output += delayed;
        }
        
        output /= 4.0;
        
        for i in 0..4 {
            let delay_samples = self.allpass_delays[i];
            let buffer_len = self.allpass_buffers[i].len();
            let read_pos = if self.allpass_write_pos[i] >= delay_samples {
                self.allpass_write_pos[i] - delay_samples
            } else {
                buffer_len - (delay_samples - self.allpass_write_pos[i])
            };
            
            let delayed = self.allpass_buffers[i][read_pos];
            let buffered = output + delayed * 0.5;
            self.allpass_buffers[i][self.allpass_write_pos[i]] = buffered;
            self.allpass_write_pos[i] = (self.allpass_write_pos[i] + 1) % buffer_len;
            output = delayed - buffered * 0.5;
        }
        
        output * self.mix + input * (1.0 - self.mix)
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.comb_delays = vec![
            (sample_rate * 0.0297) as usize,
            (sample_rate * 0.0371) as usize,
            (sample_rate * 0.0411) as usize,
            (sample_rate * 0.0437) as usize,
        ];
        self.comb_buffers = self.comb_delays.iter().map(|d| vec![0.0; *d]).collect();
        self.comb_write_pos = vec![0usize; 4];
        self.allpass_delays = vec![
            (sample_rate * 0.0197) as usize,
            (sample_rate * 0.0271) as usize,
            (sample_rate * 0.0311) as usize,
            (sample_rate * 0.0337) as usize,
        ];
        self.allpass_buffers = self.allpass_delays.iter().map(|d| vec![0.0; *d]).collect();
        self.allpass_write_pos = vec![0usize; 4];
    }
}
