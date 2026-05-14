pub struct Distortion {
    amount: f32,
    drive: f32,
}

impl Distortion {
    pub fn new() -> Self {
        Self {
            amount: 0.0,
            drive: 1.0,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.amount <= 0.001 {
            return input;
        }

        let driven = input * self.drive;
        
        let distorted = driven.tanh();
        
        let amount = self.amount;
        distorted * amount + input * (1.0 - amount)
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.clamp(0.0, 1.0);
        self.drive = 1.0 + amount * 10.0;
    }
}

impl Default for Distortion {
    fn default() -> Self {
        Self::new()
    }
}
