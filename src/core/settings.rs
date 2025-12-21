use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextSize {
    Small,
    Medium,
    Large,
}

impl TextSize {
    pub fn points(self) -> f32 {
        match self {
            TextSize::Small => 14.0,
            TextSize::Medium => 18.0,
            TextSize::Large => 24.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub text_size: TextSize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            text_size: TextSize::Medium,
        }
    }
}
