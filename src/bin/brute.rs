use std::collections::HashMap;
use std::env;

use sudoku::{console, Cell, Puzzle};
use sudoku_generator::types::{block, stats};
use sudoku_generator::utils;

fn is_ok_with_peers(val: u8, peers: &[usize], p: &Puzzle) -> bool {
    let p_vals = p.grid();
    for p_idx in peers {
        let peer_cell = &p_vals[*p_idx];
        if peer_cell.value().is_some() {
            let peer_cell_value = peer_cell.value().unwrap_or(0);
            if peer_cell_value == val {
                return false;
            }
        }
    }
    true
}

fn generate(n_iter: u32) -> (stats::Report, Puzzle) {
    println!("generate puzzle using at most {} iterations", n_iter);

    let mut puzzle = Puzzle::new(&[0; 81]);

    // list of possible values
    let vals: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // list of numbers ranging from 0 to 80 inclusive
    let grid_vals = (0..81).collect::<Vec<usize>>();

    // variable to track whether grid was solvable in the prior iteration
    let mut was_solvable = true;

    // a block is a 3x3 grid
    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    // this is where the results go
    let mut res = stats::Report::new(n_iter);

    let mut peers: HashMap<usize, Vec<usize>> = HashMap::new();

    while res.total_iter < n_iter {
        // randomly generate a value (v) and position on the board (idx)
        let v = utils::picker::pick(&vals[..]).unwrap();
        let idx = utils::picker::pick(grid_vals.as_slice()).unwrap();

        let c = Cell::new(idx, v);
        if peers.contains_key(&idx) == false {
            peers.insert(idx, c.peers());
        }
        // check peers ...
        if is_ok_with_peers(v, peers.get(&idx).unwrap().as_slice(), &puzzle) {
            match puzzle.update_cell(&c) {
                Ok(_) => (),
                Err(_) => res.num_errors += 1,
            }
        } else {
            res.num_errors += 1;
        }

        let is_solvable = blocks.iter().all(|x| x.is_valid(&puzzle));

        if puzzle.is_completed() {
            break;
        }

        if is_solvable != was_solvable {
            println!(
                "was_solvable: {},  is_solvable: {}, #iterations: {}",
                was_solvable, is_solvable, res.total_iter
            );
            was_solvable = is_solvable;
        }
        res.total_iter += 1;
    }

    res.end();
    (res, puzzle)
}

fn make_puzzle(grid: &[Cell], num_clues: u8) -> Puzzle {
    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    let mut count = 0;
    let mut puzzle: [u8; 81] = [0; 81];

    let vals: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    while count < num_clues {
        // pick a block number and a cell within that block
        let block_num = utils::picker::pick(&vals).unwrap();
        let block_val = utils::picker::pick(&vals).unwrap();
        let idx = *&blocks[block_num - 1].members()[block_val - 1];
        puzzle[idx] = grid[idx].value().unwrap_or(0);
        count += 1;
    }

    Puzzle::new(&puzzle[..])
}

fn dump_puzzle(p: &Puzzle) {
    for cell in p.grid() {
        let val = cell.value().unwrap_or(0);
        print!("{} ", val);
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };
    let (result, grid) = generate(num_iterations);

    result.print_console();

    if grid.is_completed() {
        dump_puzzle(&grid);
        for _ in 0..81 {
            let puzzle = make_puzzle(grid.grid(), 41);
            // console::utils::print_puzzle(&puzzle);
            dump_puzzle(&puzzle);
        }
    } else {
        console::utils::print_puzzle(&grid);
    }
}
