// Fill the diagonal first blocks first (blocks 1, 5, 9), then try to populate the rest.
//
//

use crate::types::{block, stats};
use crate::utils;

use sudoku::console::utils as sudoku_utils;
use sudoku::Puzzle;

pub fn remaining_values(b: &[usize], g: &[u8]) -> Vec<u8> {
    let mut m: [bool; 9] = [true; 9];

    for b_idx in b {
        let v = g[*b_idx];
        if v != 0 {
            m[v as usize - 1] = false;
        }
    }

    (0..9)
        .filter(|x| m[*x as usize])
        .map(|x| x + 1)
        .collect::<Vec<u8>>()
}

pub fn generate(n_iter: u32) -> (stats::Report, Puzzle) {
    println!("generate diag started.");

    let mut grid: [u8; 81] = [0; 81];

    for i in [0, 30, 60].iter() {
        let block = block::members_for_idx(*i);
        for idx in block.iter() {
            let remaining = remaining_values(&block, &grid);
            grid[*idx] = utils::pick(&remaining).unwrap();
        }
    }

    sudoku_utils::print_console(&grid, 9, 9);
    super::base::generate(n_iter, &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        generate(1000);
    }
}
