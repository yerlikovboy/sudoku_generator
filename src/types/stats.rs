use std::time::Instant;

pub struct Report {
    pub num_max_iter: u32,
    pub total_iter: u32,
    pub started: Instant,
    pub ended: Instant,
    pub num_errors: u32,
    pub num_overwrites: u32,
}

impl Report {
    pub fn new(max_iter: u32) -> Report {
        Report {
            num_max_iter: max_iter,
            total_iter: 0,
            started: Instant::now(),
            ended: Instant::now(),
            num_errors: 0,
            num_overwrites: 0,
        }
    }

    pub fn end(&mut self) {
        self.ended = Instant::now();
    }

    pub fn print_console(&self) {
        println!(
            "duration: {}ms, #iterations: {}, #update errrors: {}",
            self.started.elapsed().as_millis(),
            self.total_iter,
            self.num_errors,
        );
    }
}
