use crate::cmd;
use crate::job::result;
use crate::types::block;
use crate::utils;
use sudoku::{Cell, Puzzle};

use std::collections::HashMap;

pub fn generate(cfg: &cmd::Config, seed: &[u8]) -> result::Report {
    let mut puzzle = Puzzle::new(seed);

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
    let mut res = result::Report::new(cfg);

    res.set_seed(seed.to_vec());

    let mut peers: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut iter = 0;

    while iter < cfg.n_iterations() {
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
                Err(_) => res.incr_error_count(),
            }
        } else {
            res.incr_error_count();
        }

        let is_solvable = blocks.iter().all(|x| x.is_valid(&puzzle));

        if is_solvable != was_solvable {
            res.add_state_change(iter, was_solvable, is_solvable);
            was_solvable = is_solvable;
        }

        if puzzle.is_completed() {
            break;
        }

        iter += 1;
    }

    let g = puzzle
        .grid()
        .iter()
        .map(|x| x.value().unwrap_or(0))
        .collect::<Vec<u8>>();
    res.set_grid(g);
    res.set_total_iter(iter);

    res
}
