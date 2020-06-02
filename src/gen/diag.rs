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

    /*
    let mut puzzle = Puzzle::new(&grid);
    sudoku_utils::print_puzzle(&puzzle);

    // list of possible values
    let vals: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // list of numbers ranging from 0 to 80 inclusive
    let grid_vals = (0..81).collect::<Vec<usize>>();
    // a block is a 3x3 grid
    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    // this is where the results go
    let mut res = stats::Report::new(n_iter);

    let mut peers: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut iter_count = 0;
    while iter_count < n_iter {
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
        iter_count += 1;
    }

    res.end();
    res.print_console();
    sudoku_utils::print_puzzle(&puzzle);
    println!("generate diag ended.");
    puzzle
        */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        generate();
    }
}
