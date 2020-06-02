use crate::types::stats;
use sudoku::Puzzle;

pub fn generate(n_iter: u32) -> (stats::Report, Puzzle) {
    super::base::generate(n_iter, &[0; 81])
}
