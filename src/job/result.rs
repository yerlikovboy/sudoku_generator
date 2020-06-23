use crate::cmd::{Algorithm, Config};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    alg: Algorithm,

    max_iter: u32,
    total_iter: u32,

    timestamp_millis: u128,

    error_count: u32,
    overwrite_count: u32,

    state_changes: Vec<StateChange>,
    seed: Option<Vec<u8>>,
    grid: Option<Vec<u8>>,
    puzzle_complete: bool,
}

impl Report {
    pub fn new(cfg: &Config) -> Report {
        Report {
            max_iter: cfg.n_iterations(),
            total_iter: 0,
            error_count: 0,
            overwrite_count: 0,
            state_changes: Vec::new(),
            seed: None,
            grid: None,
            puzzle_complete: false,
            alg: cfg.algorithm(),
            timestamp_millis: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }

    pub fn set_seed(&mut self, seed: Vec<u8>) {
        self.seed = Some(seed);
    }

    pub fn set_grid(&mut self, grid: Vec<u8>) {
        self.grid = Some(grid);
    }

    pub fn grid(&self) -> Option<&Vec<u8>> {
        self.grid.as_ref()
    }

    pub fn incr_overwrite_count(&mut self) {
        self.overwrite_count += 1;
    }

    pub fn incr_error_count(&mut self) {
        self.error_count += 1;
    }

    pub fn add_state_change(&mut self, iter: u32, prev_ok: bool, ok: bool) {
        self.state_changes.push(StateChange {
            iter,
            previous_ok: prev_ok,
            is_ok: ok,
        });
    }

    pub fn set_total_iter(&mut self, n: u32) {
        self.total_iter = n;
        self.puzzle_complete = self.is_grid_complete();
    }

    fn is_grid_complete(&self) -> bool {
        if self.grid.is_none() {
            return false;
        }

        self.grid.as_ref().unwrap().iter().all(|x| *x != 0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateChange {
    iter: u32,
    previous_ok: bool,
    is_ok: bool,
}
