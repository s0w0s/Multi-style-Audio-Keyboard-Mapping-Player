use anyhow::Result;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct Sample {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration: f32,
}

pub struct SampleManager {
    pub samples: [Option<Arc<RwLock<Sample>>>; 11],
}

impl SampleManager {
    pub fn new() -> Self {
        Self {
            samples: [None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn load_sample(&mut self, style_index: usize, path: &Path) -> Result<()> {
        if style_index >= 11 {
            anyhow::bail!("Style index must be 0-10");
        }

        let file = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            hint.with_extension(ext.to_str().unwrap_or(""));
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let mut format = probed.format;
        let track = format.default_track().ok_or_else(|| anyhow::anyhow!("No default track"))?;
        let decoder_opts = DecoderOptions::default();

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)?;

        let mut all_samples: Vec<f32> = Vec::new();
        let mut sample_rate = 44100u32;
        let mut channels = 2u16;

        let track_id = track.id;
        
        loop {
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            let decoded = match decoder.decode(&packet) {
                Ok(d) => d,
                Err(_) => continue,
            };

            if sample_rate == 44100 {
                sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
            }

            match decoded {
                symphonia::core::audio::AudioBufferRef::F32(buf) => {
                    channels = buf.spec().channels.count() as u16;
                    for plane in buf.planes().planes() {
                        for &sample in plane.iter() {
                            all_samples.push(sample);
                        }
                    }
                }
                symphonia::core::audio::AudioBufferRef::S16(buf) => {
                    channels = buf.spec().channels.count() as u16;
                    for plane in buf.planes().planes() {
                        for &sample in plane.iter() {
                            all_samples.push(sample as f32 / 32768.0);
                        }
                    }
                }
                _ => {}
            }
        }

        let duration = all_samples.len() as f32 / (sample_rate as f32 * channels as f32);

        let sample = Sample {
            data: all_samples,
            sample_rate,
            channels,
            duration,
        };

        self.samples[style_index] = Some(Arc::new(RwLock::new(sample)));
        log::info!("Loaded sample for style {}: {:.2}s, {}Hz, {}ch", 
                   style_index, duration, sample_rate, channels);

        Ok(())
    }

    pub fn load_directory(&mut self, dir_path: &Path) -> Result<()> {
        let mut entries: Vec<_> = std::fs::read_dir(dir_path)?
            .filter_map(|e| e.ok())
            .collect();
        
        entries.sort_by_key(|e| e.file_name());
        
        for (i, entry) in entries.iter().enumerate().take(11) {
            let path = entry.path();
            if path.is_file() {
                if let Err(e) = self.load_sample(i, &path) {
                    log::warn!("Failed to load {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(())
    }

    pub fn get_sample(&self, style_index: usize) -> Option<Arc<RwLock<Sample>>> {
        self.samples.get(style_index).and_then(|s| s.clone())
    }

    pub fn get_duration(&self) -> f32 {
        self.samples[0]
            .as_ref()
            .map(|s| s.read().duration)
            .unwrap_or(0.0)
    }
}

impl Default for SampleManager {
    fn default() -> Self {
        Self::new()
    }
}
