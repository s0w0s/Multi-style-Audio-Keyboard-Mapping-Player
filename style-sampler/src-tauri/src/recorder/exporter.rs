use anyhow::Result;
use hound::{WavWriter, WavSpec, SampleFormat};
use std::fs::File;
use std::io::BufWriter;

pub struct Exporter {
    sample_rate: u32,
    channels: u16,
    samples: Vec<f32>,
}

impl Exporter {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            samples: Vec::new(),
        }
    }

    pub fn write_sample(&mut self, left: f32, right: f32) {
        self.samples.push(left);
        self.samples.push(right);
    }

    pub fn export(&self, path: &str) -> Result<()> {
        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut wav_writer = WavWriter::new(writer, spec)?;

        for &sample in &self.samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            wav_writer.write_sample(sample_i16)?;
        }

        wav_writer.finalize()?;
        
        log::info!("Exported {} samples to {}", self.samples.len() / 2, path);
        
        Ok(())
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }
}
