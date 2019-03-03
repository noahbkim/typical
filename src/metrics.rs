use std::time::Instant;


pub struct Metrics {
    mistakes: u32,
    start: Instant,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics { mistakes: 0, start: Instant::now() }
    }
}
