use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub trigger_mode: String,
    pub loop_mode: String,
    pub loop_start: f32,
    pub volume: f32,
    pub bpm: f32,
    pub transition_type: String,
    pub transition_time: f32,
    pub effects: EffectsSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsSettings {
    pub filter_cutoff: f32,
    pub reverb_mix: f32,
    pub delay_time: f32,
    pub delay_feedback: f32,
    pub distortion_amount: f32,
    pub chorus_mix: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            trigger_mode: "gate".to_string(),
            loop_mode: "while_pressed".to_string(),
            loop_start: 0.0,
            volume: 0.8,
            bpm: 128.0,
            transition_type: "crossfade".to_string(),
            transition_time: 50.0,
            effects: EffectsSettings {
                filter_cutoff: 10000.0,
                reverb_mix: 0.0,
                delay_time: 0.3,
                delay_feedback: 0.4,
                distortion_amount: 0.0,
                chorus_mix: 0.0,
            },
        }
    }
}

impl Settings {
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&json)?;
        Ok(settings)
    }
}
