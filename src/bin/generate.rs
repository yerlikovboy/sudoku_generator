use std::env;

//use sudoku::console;
use sudoku_generator::cmd;
use sudoku_generator::cmd::{Algorithm, Config};
use sudoku_generator::gen::{brute, diag};
use sudoku_generator::job::result;

use serde_json;

fn execute(c: Config) -> result::Report {
    match c.algorithm() {
        Algorithm::Brute => brute::generate(c.n_iterations()),
        Algorithm::Diagonal => diag::generate(c.n_iterations()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match cmd::parse_args(args) {
        Ok(cfg) => {
            let result = execute(cfg);
            let r_json = serde_json::to_string(&result).unwrap();
            //println!("{:?}", result);
            println!("{}", r_json);
            // console::utils::print_console(result.grid().unwrap().as_slice(), 9, 9);
        }
        Err(_) => println!("usage: generate <alg> <num iterations>"),
    }
}
