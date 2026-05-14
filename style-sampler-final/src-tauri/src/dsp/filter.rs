pub struct LowPassFilter {
    cutoff: f32,
    sample_rate: f32,
    alpha: f32,
    y_prev: f32,
}

impl LowPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let alpha = Self::calculate_alpha(cutoff, sample_rate);
        Self {
            cutoff,
            sample_rate,
            alpha,
            y_prev: 0.0,
        }
    }

    fn calculate_alpha(cutoff: f32, sample_rate: f32) -> f32 {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / sample_rate;
        dt / (rc + dt)
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        self.y_prev += self.alpha * (input - self.y_prev);
        self.y_prev
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(20.0).min(20000.0);
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }
}

pub struct HighPassFilter {
    cutoff: f32,
    sample_rate: f32,
    alpha: f32,
    x_prev: f32,
    y_prev: f32,
}

impl HighPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let alpha = Self::calculate_alpha(cutoff, sample_rate);
        Self {
            cutoff,
            sample_rate,
            alpha,
            x_prev: 0.0,
            y_prev: 0.0,
        }
    }

    fn calculate_alpha(cutoff: f32, sample_rate: f32) -> f32 {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / sample_rate;
        rc / (rc + dt)
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.y_prev + input - self.x_prev);
        self.x_prev = input;
        self.y_prev = output;
        output
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(20.0).min(20000.0);
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }
}
