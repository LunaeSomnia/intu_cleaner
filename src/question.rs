#[derive(Debug, Clone)]
pub struct Question {
    desc: String,
    options: [String; 4],
    correct: usize,
}

impl Question {
    pub fn new(desc: String, options: [String; 4], correct: usize) -> Self {
        Self {
            desc,
            options,
            correct,
        }
    }

    pub fn desc(&self) -> &str {
        self.desc.as_ref()
    }

    pub fn options(&self) -> &[String; 4] {
        &self.options
    }

    pub fn correct(&self) -> usize {
        self.correct
    }

    pub fn set_correct(&mut self, correct: usize) {
        self.correct = correct;
    }
}
