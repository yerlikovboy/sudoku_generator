pub mod cmd;
pub mod gen;
pub mod job;
pub mod types;
pub mod utils;

pub mod console;
pub mod writer;

use crate::types::block;
use sudoku::{Cell, Puzzle};

pub fn make_puzzle(grid: &[Cell], num_clues: u8) -> Puzzle {
    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    let mut count = 0;
    let mut puzzle: [u8; 81] = [0; 81];

    let vals: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    while count < num_clues {
        // pick a block number and a cell within that block
        let block_num = utils::pick(&vals).unwrap();
        let block_val = utils::pick(&vals).unwrap();
        let idx = *&blocks[block_num - 1].members()[block_val - 1];
        puzzle[idx] = grid[idx].value().unwrap_or(0);
        count += 1;
    }

    Puzzle::new(&puzzle[..])
}
