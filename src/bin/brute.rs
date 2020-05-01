use std::env;

use sudoku::game::actions::Move;
use sudoku::{console, Cell, Puzzle};
use sudoku_misc::types::{block, stats};
use sudoku_misc::utils;

fn generate(n_iter: u32) -> stats::Report {
    println!("generate puzzle using at most {} iterations", n_iter);

    let mut puzzle = Puzzle::new(&[0; 81]);

    let vals: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let grid_vals = (0..81).collect::<Vec<usize>>();

    let mut was_ok = true;

    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    let mut res = stats::Report::new(n_iter);

    while res.total_iter < n_iter {
        let v = utils::picker::pick(&vals[..]).unwrap();
        let idx = utils::picker::pick(grid_vals.as_slice());

        let c = Cell::from_grid_idx(idx.unwrap());
        let m = Move::new(res.total_iter, c.row, c.column, v);
        match puzzle.update_cell(&m) {
            Ok(_) => (),
            Err(_) => res.num_errors += 1,
        }

        if puzzle.is_completed() {
            res.is_complete = true;
            break;
        }

        res.is_solvable = blocks.iter().all(|x| x.is_valid(&puzzle));
        if res.is_solvable != was_ok {
            println!(
                "is ok: {}, #iterations: {}",
                res.is_solvable, res.total_iter
            );
            was_ok = res.is_solvable;
        }
        res.total_iter += 1;
    }
    res.copy_grid(puzzle.grid_as_ref()).unwrap();

    console::utils::print_puzzle(&puzzle);

    res
}

fn make_puzzle(grid: &[u8], num_clues: u8) -> Puzzle {
    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    let mut count = 0;
    let mut puzzle: [u8; 81] = [0; 81];

    while count < num_clues {
        let vals: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        // pick a block number and a cell within that block
        let block_num = utils::picker::pick(&vals).unwrap();
        let block_val = utils::picker::pick(&vals).unwrap();
        let idx = *&blocks[block_num - 1].members()[block_val - 1];
        puzzle[idx] = grid[idx];
        count += 1;
    }

    Puzzle::new(&puzzle[..])
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };
    let result = generate(num_iterations);
    result.print_console();

    if result.is_complete {
        let puzzle = make_puzzle(result.grid.as_slice(), 41);
        console::utils::print_puzzle(&puzzle);
    }
}
