use rand::prelude::IndexedRandom;
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
