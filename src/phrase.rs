use std::time::Instant;

pub struct Phrase {
    text: String,
    progress: u32,
    wrong: bool,
    mistakes: u32,
    start: Instant
}

impl Phrase {
    pub fn new(text: String) -> Phrase {
        Phrase {
            text,
            progress: 0,
            wrong: false,
            mistakes: 0,
            start: Instant::now()
        }
    }
}
