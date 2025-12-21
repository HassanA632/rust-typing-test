use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestState {
    Waiting,
    Running,
    Finished,
}

pub struct TestSession {
    pub words: Vec<String>,
    pub current_index: usize,
    pub input: String,
    pub state: TestState,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub correct_words: usize,
}

impl TestSession {
    pub fn new(words: Vec<String>) -> Self {
        Self {
            words,
            current_index: 0,
            input: String::new(),
            state: TestState::Waiting,
            started_at: None,
            finished_at: None,
            correct_words: 0,
        }
    }

    pub fn expected_word(&self) -> Option<&str> {
        self.words.get(self.current_index).map(|s| s.as_str())
    }

    pub fn next_preview(&self, count: usize) -> Vec<&str> {
        self.words
            .iter()
            .skip(self.current_index)
            .take(count)
            .map(|s| s.as_str())
            .collect()
    }

    pub fn on_first_input_if_needed(&mut self) {
        if self.state == TestState::Waiting {
            self.state = TestState::Running;
            self.started_at = Some(Instant::now());
        }
    }

    pub fn submit_current_word(&mut self) {
        if self.state == TestState::Waiting {
            return;
        }
        if self.state == TestState::Finished {
            return;
        }

        let expected = self.expected_word().unwrap_or("");
        let typed = self.input.trim();

        if typed == expected {
            self.correct_words += 1;
        }

        self.input.clear();
        self.current_index += 1;

        if self.current_index >= self.words.len() {
            self.state = TestState::Finished;
            self.finished_at = Some(Instant::now());
        }
    }

    pub fn elapsed(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at, self.state) {
            (Some(start), Some(end), _) => Some(end.duration_since(start)),
            (Some(start), None, TestState::Running) => Some(start.elapsed()),
            _ => None,
        }
    }

    pub fn wpm(&self) -> Option<f32> {
        let elapsed = self.elapsed()?.as_secs_f32();
        if elapsed <= 0.0 {
            return None;
        }
        let minutes = elapsed / 60.0;
        Some(self.correct_words as f32 / minutes)
    }
}
