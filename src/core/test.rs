use std::time::Duration;
use web_time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestState {
    Waiting,
    Running,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitEvent {
    None,
    Wrong { expected: String },
    CorrectFirstTry { expected: String },
    CorrectAfterMistake { expected: String },
}

pub struct TestSession {
    pub words: Vec<String>,
    pub current_index: usize,
    pub input: String,
    pub state: TestState,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub correct_words: usize,
    pub last_feedback: Option<String>,
    pub should_focus_input: bool,
    pub result_saved: bool,
    pub current_word_had_mistake: bool,
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
            last_feedback: None,
            should_focus_input: true,
            result_saved: false,
            current_word_had_mistake: false,
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

    pub fn attempt_submit_current_word(&mut self) -> SubmitEvent {
        if self.state == TestState::Waiting || self.state == TestState::Finished {
            return SubmitEvent::None;
        }

        let expected = self.expected_word().unwrap_or("").to_string();
        let typed = self.input.trim();

        // Don't allow submitting empty (prevents accidental space spam)
        if typed.is_empty() {
            self.last_feedback = Some("Type the word above".to_string());
            return SubmitEvent::None;
        }

        if typed == expected {
            self.correct_words += 1;
            self.last_feedback = None;

            // Decide whether this was clean or after a mistake
            let event = if self.current_word_had_mistake {
                SubmitEvent::CorrectAfterMistake {
                    expected: expected.clone(),
                }
            } else {
                SubmitEvent::CorrectFirstTry {
                    expected: expected.clone(),
                }
            };

            self.input.clear();
            self.current_index += 1;
            self.current_word_had_mistake = false; // reset for the next word

            if self.current_index >= self.words.len() {
                self.state = TestState::Finished;
                self.finished_at = Some(Instant::now());
            }

            event
        } else {
            // Wrong = reset input and force retry
            self.last_feedback = Some("Try again".to_string());
            self.input.clear();
            self.current_word_had_mistake = true; // mark that this word had a mistake
            SubmitEvent::Wrong { expected }
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
