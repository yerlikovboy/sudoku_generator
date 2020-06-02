use std::env;

use sudoku::console;
use sudoku_generator::gen::brute;
use sudoku_generator::utils;

/*
fn make_puzzle(grid: &[Cell], num_clues: u8) -> Puzzle {
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
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };
    let (result, grid) = brute::generate(num_iterations);

    result.print_console();

    if grid.is_completed() {
        utils::dump_puzzle(&grid);
        for _ in 0..81 {
            let puzzle = sudoku_generator::make_puzzle(grid.grid(), 41);
            // console::utils::print_puzzle(&puzzle);
            utils::dump_puzzle(&puzzle);
        }
    } else {
        console::utils::print_puzzle(&grid);
    }
}
