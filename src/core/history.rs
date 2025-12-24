use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultEntry {
    pub wpm: f32,
    pub elapsed_secs: f32,
    pub correct_words: u32,
    pub total_words: u32,
}
