use cpal::traits::{DeviceTrait, HostTrait};

pub struct AudioOutput {
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioOutput {
    pub fn new() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = device.default_output_config().ok()?;
        
        Some(Self {
            sample_rate: config.sample_rate().0,
            channels: config.channels(),
        })
    }
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            sample_rate: 44100,
            channels: 2,
        })
    }
}
