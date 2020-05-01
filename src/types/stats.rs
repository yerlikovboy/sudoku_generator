use std::time::Instant;

pub struct Report {
    pub num_max_iter: u32,
    pub total_iter: u32,
    pub started: Instant,
    pub ended: Instant,
    pub num_errors: u32,
    pub num_overwrites: u32,
    pub is_complete: bool,
    pub is_solvable: bool,
    pub grid: Vec<u8>,
}

impl Report {
    pub fn new(n: u32) -> Report {
        Report {
            num_max_iter: n,
            total_iter: 0,
            started: Instant::now(),
            ended: Instant::now(),
            num_errors: 0,
            num_overwrites: 0,
            is_complete: false,
            is_solvable: false,
            grid: Vec::with_capacity(81),
        }
    }
    pub fn copy_grid(&mut self, grid: &[u8]) -> Result<(), String> {
        assert_eq!(grid.len(), 81);
        (0..81).for_each(|x| self.grid.push(grid[x]));
        Ok(())
    }

    pub fn print_console(&self) {
        println!(
            "duration: {}ms, #iterations: {}, #update errrors: {}, is_complete:{}, is solvable: {}",
            self.started.elapsed().as_millis(),
            self.total_iter,
            self.num_errors,
            self.is_complete,
            self.is_solvable,
        );
        if self.is_complete {
            println!("{:?}", self.grid);
        }
    }
}
