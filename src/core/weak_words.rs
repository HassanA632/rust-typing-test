use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type WeakWords = HashMap<String, u8>;

pub const WEAK_MAX: u8 = 10;

// Increase weakness on a wrong attempt (+2), capped at WEAK_MAX
pub fn bump_word(map: &mut WeakWords, word: &str) {
    let entry = map.entry(word.to_string()).or_insert(0);
    *entry = (*entry).saturating_add(2).min(WEAK_MAX);
}

/// Decrease weakness on a clean first-try correct (-1) capped at 0, If reaches 0 remove it from the map
pub fn reward_clean_correct(map: &mut WeakWords, word: &str) {
    if let Some(v) = map.get_mut(word) {
        *v = v.saturating_sub(1);
        if *v == 0 {
            map.remove(word);
        }
    }
}
