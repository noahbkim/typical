use std::time::Instant;


pub struct Metrics {
    pub mistakes: u32,
    pub start: Instant,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics { mistakes: 0, start: Instant::now() }
    }
}
