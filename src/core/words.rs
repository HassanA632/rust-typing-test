use rand::prelude::IndexedRandom;

static WORDS: &[&str] = &[
    "the", "of", "and", "to", "a", "in", "is", "you", "that", "it", "he", "was", "for", "on",
    "are", "as", "with", "his", "they",
    "i",
    // we’ll replace this with the full top-400 list next commit
];

pub fn generate_prompt(count: usize) -> Vec<String> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| WORDS.choose(&mut rng).unwrap().to_string())
        .collect()
}
