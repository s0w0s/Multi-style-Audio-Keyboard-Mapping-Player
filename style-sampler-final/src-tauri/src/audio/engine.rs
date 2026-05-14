use crate::audio::sample::SampleManager;
use crate::audio::playhead::Playhead;
use crate::dsp::{LowPassFilter, HighPassFilter, Reverb, Delay, Distortion, Chorus};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

pub struct AudioEngine {
    pub sample_manager: Arc<RwLock<SampleManager>>,
    pub playhead: Arc<Playhead>,
    active_style: Arc<AtomicUsize>,
    pub master_volume: Arc<RwLock<f32>>,
    key_pressed: Arc<AtomicBool>,
    pub trigger_mode: Arc<RwLock<TriggerMode>>,
    pub loop_mode: Arc<RwLock<LoopMode>>,
    pub transition_type: Arc<RwLock<TransitionType>>,
    pub transition_time: Arc<RwLock<f32>>,
    lowpass: Arc<RwLock<LowPassFilter>>,
    highpass: Arc<RwLock<HighPassFilter>>,
    reverb: Arc<RwLock<Reverb>>,
    delay: Arc<RwLock<Delay>>,
    distortion: Arc<RwLock<Distortion>>,
    chorus: Arc<RwLock<Chorus>>,
}

unsafe impl Send for AudioEngine {}
unsafe impl Sync for AudioEngine {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriggerMode {
    Gate,
    Trigger,
    Loop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopMode {
    Off,
    WhilePressed,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransitionType {
    Hard,
    Fade,
    Crossfade,
}

impl AudioEngine {
    pub fn new() -> Self {
        let sample_rate = 44100;
        
        Self {
            sample_manager: Arc::new(RwLock::new(SampleManager::new())),
            playhead: Arc::new(Playhead::new()),
            active_style: Arc::new(AtomicUsize::new(11)),
            master_volume: Arc::new(RwLock::new(0.8)),
            key_pressed: Arc::new(AtomicBool::new(false)),
            trigger_mode: Arc::new(RwLock::new(TriggerMode::Gate)),
            loop_mode: Arc::new(RwLock::new(LoopMode::WhilePressed)),
            transition_type: Arc::new(RwLock::new(TransitionType::Crossfade)),
            transition_time: Arc::new(RwLock::new(0.05)),
            lowpass: Arc::new(RwLock::new(LowPassFilter::new(10000.0, sample_rate as f32))),
            highpass: Arc::new(RwLock::new(HighPassFilter::new(20.0, sample_rate as f32))),
            reverb: Arc::new(RwLock::new(Reverb::new(sample_rate as f32))),
            delay: Arc::new(RwLock::new(Delay::new(sample_rate as f32))),
            distortion: Arc::new(RwLock::new(Distortion::new())),
            chorus: Arc::new(RwLock::new(Chorus::new(sample_rate as f32))),
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device found"))?;
        
        log::info!("Using audio device: {}", device.name().unwrap_or_default());

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        log::info!("Audio config: {}Hz, {} channels", sample_rate, channels);

        self.lowpass.write().update_sample_rate(sample_rate as f32);
        self.highpass.write().update_sample_rate(sample_rate as f32);
        self.reverb.write().update_sample_rate(sample_rate as f32);
        self.delay.write().update_sample_rate(sample_rate as f32);
        self.chorus.write().update_sample_rate(sample_rate as f32);

        let sample_manager = self.sample_manager.clone();
        let playhead = self.playhead.clone();
        let active_style = self.active_style.clone();
        let master_volume = self.master_volume.clone();
        let key_pressed = self.key_pressed.clone();
        let loop_mode = self.loop_mode.clone();
        let lowpass = self.lowpass.clone();
        let highpass = self.highpass.clone();
        let reverb = self.reverb.clone();
        let delay = self.delay.clone();
        let distortion = self.distortion.clone();
        let chorus = self.chorus.clone();

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let frame_count = data.len() / channels;
                let dt = frame_count as f32 / sample_rate as f32;
                
                let is_playing = playhead.is_playing();
                
                if is_playing {
                    let position = playhead.get_position();
                    let duration = *playhead.total_duration.read();
                    
                    if position >= duration {
                        let lm = *loop_mode.read();
                        if lm != LoopMode::Off && key_pressed.load(Ordering::SeqCst) {
                            playhead.set_position(*playhead.loop_start.read());
                        } else {
                            playhead.pause();
                        }
                    } else {
                        playhead.advance(dt);
                    }
                }
                
                let volume = *master_volume.read();
                let style = active_style.load(Ordering::SeqCst);
                
                let sm = sample_manager.read();
                
                if let Some(sample_arc) = sm.get_sample(style) {
                    let sample = sample_arc.read();
                    let position = playhead.get_position();
                    let start_sample = (position * sample.sample_rate as f32 * sample.channels as f32) as usize;
                    
                    let sample_data = &sample.data;
                    
                    for frame in 0..frame_count {
                        let sample_idx = start_sample + frame * sample.channels as usize;
                        
                        let mut out_sample = if sample_idx < sample_data.len() - 1 {
                            let left = sample_data[sample_idx];
                            let right = if sample_idx + 1 < sample_data.len() {
                                sample_data[sample_idx + 1]
                            } else {
                                left
                            };
                            (left + right) * 0.5 * volume
                        } else {
                            0.0
                        };
                        
                        let mut mono_sample = out_sample;
                        
                        mono_sample = lowpass.write().process_sample(mono_sample);
                        mono_sample = highpass.write().process_sample(mono_sample);
                        mono_sample = reverb.write().process_sample(mono_sample);
                        mono_sample = delay.write().process_sample(mono_sample);
                        mono_sample = distortion.write().process_sample(mono_sample);
                        mono_sample = chorus.write().process_sample(mono_sample);
                        
                        out_sample = mono_sample;
                        
                        data[frame * channels] = out_sample;
                        if channels > 1 {
                            data[frame * channels + 1] = out_sample;
                        }
                    }
                } else {
                    for frame in 0..frame_count {
                        for ch in 0..channels {
                            data[frame * channels + ch] = 0.0;
                        }
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        
        log::info!("Audio engine initialized successfully");
        Ok(())
    }

    pub fn play_style(&self, style_index: usize) {
        if style_index >= 11 {
            log::warn!("Invalid style index: {}", style_index);
            return;
        }
        
        let tm = *self.trigger_mode.read();
        
        match tm {
            TriggerMode::Gate => {
                if !self.playhead.is_playing() {
                    self.playhead.start();
                }
                self.active_style.store(style_index, Ordering::SeqCst);
                self.key_pressed.store(true, Ordering::SeqCst);
            }
            TriggerMode::Trigger => {
                self.active_style.store(style_index, Ordering::SeqCst);
                self.playhead.start();
                self.key_pressed.store(false, Ordering::SeqCst);
            }
            TriggerMode::Loop => {
                self.active_style.store(style_index, Ordering::SeqCst);
                self.playhead.start();
                self.playhead.set_loop_enabled(true);
                self.key_pressed.store(false, Ordering::SeqCst);
            }
        }
        
        log::debug!("Playing style {}, trigger mode: {:?}", style_index, tm);
    }

    pub fn stop(&self) {
        self.playhead.stop();
        self.key_pressed.store(false, Ordering::SeqCst);
    }

    pub fn set_volume(&self, volume: f32) {
        *self.master_volume.write() = volume.clamp(0.0, 1.0);
    }

    pub fn set_bpm(&self, _bpm: f32) {}

    pub fn set_loop_mode(&self, mode: LoopMode) {
        *self.loop_mode.write() = mode;
        match mode {
            LoopMode::WhilePressed | LoopMode::Always => {
                self.playhead.set_loop_enabled(true);
            }
            LoopMode::Off => {
                self.playhead.set_loop_enabled(false);
            }
        }
    }

    pub fn set_loop_start(&self, position: f32) {
        self.playhead.set_loop_start(position);
    }

    pub fn set_trigger_mode(&self, mode: TriggerMode) {
        *self.trigger_mode.write() = mode;
    }

    pub fn set_effect_param(&self, effect: &str, _param: &str, value: f32) {
        match effect {
            "filter" => {
                self.lowpass.write().set_cutoff(value);
            }
            "reverb" => {
                self.reverb.write().set_mix(value / 100.0);
            }
            "delay" => {
                self.delay.write().set_time(value / 100.0);
            }
            "distortion" => {
                self.distortion.write().set_amount(value / 100.0);
            }
            "chorus" => {
                self.chorus.write().set_mix(value / 100.0);
            }
            _ => {}
        }
    }

    pub fn get_playhead_position(&self) -> f32 {
        self.playhead.get_position()
    }

    pub fn set_playhead_position(&self, position: f32) {
        self.playhead.set_position(position);
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
