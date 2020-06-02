use crate::types::{block, stats};
use crate::utils;
use sudoku::{Cell, Puzzle};

use std::collections::HashMap;

pub fn generate(n_iter: u32, starter_grid: &[u8]) -> (stats::Report, Puzzle) {
    println!("generate puzzle using at most {} iterations", n_iter);

    let mut puzzle = Puzzle::new(starter_grid);

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
        let v = utils::pick(&vals[..]).unwrap();
        let idx = utils::pick(grid_vals.as_slice()).unwrap();

        let c = Cell::new(idx, v);
        if peers.contains_key(&idx) == false {
            peers.insert(idx, c.peers());
        }
        // check peers ...
        if utils::is_ok_with_peers(v, peers.get(&idx).unwrap().as_slice(), &puzzle) {
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
