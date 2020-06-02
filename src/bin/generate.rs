use std::env;

use sudoku::console;
use sudoku::Puzzle;
use sudoku_generator::cmd;
use sudoku_generator::cmd::{Algorithm, Config};
use sudoku_generator::gen::{brute, diag};
use sudoku_generator::types::stats;

fn execute(c: Config) -> (stats::Report, Puzzle) {
    match c.algorithm() {
        Algorithm::Brute => brute::generate(c.n_iterations()),
        Algorithm::Diagonal => diag::generate(c.n_iterations()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match cmd::parse_args(args) {
        Ok(cfg) => {
            let (result, grid) = execute(cfg);
            result.print_console();
            console::utils::print_puzzle(&grid);
        }
        Err(_) => println!("usage: generate <alg> <num iterations>"),
    }
}
