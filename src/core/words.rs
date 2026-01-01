use crate::core::weak_words::WeakWords;
use rand::{Rng, prelude::IndexedRandom};
use std::sync::OnceLock;

static WORDS: OnceLock<Vec<String>> = OnceLock::new();

fn load_words() -> &'static Vec<String> {
    WORDS.get_or_init(|| {
        let raw = include_str!("../../assets/top500_words.txt");

        let mut words: Vec<String> = raw
            .lines()
            .map(|w| w.trim().to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();

        // remove dupes while preserving order
        words.dedup();

        words
    })
}

pub fn generate_prompt(count: usize) -> Vec<String> {
    let words = load_words();
    assert!(
        !words.is_empty(),
        "Word list is empty. Check assets/top500_words.txt"
    );

    let mut rng = rand::rng();

    (0..count)
        .map(|_| words.choose(&mut rng).expect("word list empty").clone())
        .collect()
}

pub fn generate_practice_prompt(count: usize, weak: &WeakWords) -> Vec<String> {
    let all = load_words();
    assert!(!all.is_empty(), "Word list empty");

    // Build a weighted pool of weak words
    let mut weak_pool: Vec<&str> = Vec::new();
    for (w, score) in weak.iter() {
        if *score > 0 {
            for _ in 0..*score {
                weak_pool.push(w.as_str());
            }
        }
    }

    let mut rng = rand::rng();
    let have_weak = !weak_pool.is_empty();

    (0..count)
        .map(|_| {
            let pick_weak = have_weak && rng.random_bool(0.7); // 70% weak words
            if pick_weak {
                weak_pool.choose(&mut rng).unwrap().to_string()
            } else {
                all.choose(&mut rng).unwrap().clone()
            }
        })
        .collect()
}
