use std::time::Instant;


pub struct Phrase {
    pub text: String,
    pub cursor: usize,
    pub wrong: bool,
    pub mistakes: u32,
    pub start: Instant,
}

impl Phrase {
    pub fn new() -> Phrase {
        Phrase {text: String::new(), cursor: 0, wrong: false, mistakes: 0, start: Instant::now()}
    }

    pub fn next(&mut self, text: String) {
        self.text = text;
        self.cursor = 0;
        self.wrong = false;
        self.mistakes = 0;
        self.start = Instant::now();
    }

    pub fn text(&mut self, text: String) {
        if text.len() > 0 && text.chars().next().unwrap() == self.text.chars().nth(self.cursor).unwrap() {
            self.wrong = false;
            self.cursor += 1;
        } else {
            self.wrong = true;
            self.mistakes += 1;
        }
    }

    pub fn done(&self) -> bool {
        self.cursor == self.text.len()
    }
}
