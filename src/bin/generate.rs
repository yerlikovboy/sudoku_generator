use std::env;

use sudoku::console;
use sudoku::Puzzle;
use sudoku_generator::gen::{brute, diag};
use sudoku_generator::types::stats;

enum Algorithm {
    Brute,
    Diagonal,
}
struct Config {
    n_iterations: u32,
    alg: Algorithm,
}

/*
impl Config {
    fn default() -> Config {
        Config {
            n_iterations: 10000,
            alg: Algorithm::Brute,
        }
    }

    fn new(n_iterations: u32, alg: Algorithm) -> Config {
        Config { n_iterations, alg }
    }
}
*/

fn execute(c: Config) -> (stats::Report, Puzzle) {
    match c.alg {
        Algorithm::Brute => brute::generate(c.n_iterations),
        Algorithm::Diagonal => diag::generate(c.n_iterations),
    }
}

fn parse_args(args: Vec<String>) -> Result<Config, String> {
    if args.len() != 3 {
        return Err(String::from("not enough arguments"));
    }

    let alg_type_str = args[1].as_str().to_ascii_lowercase();

    Ok(Config {
        n_iterations: *&args[2].parse::<u32>().unwrap(),
        alg: match alg_type_str.as_str() {
            "diag" => Algorithm::Diagonal,
            _ => Algorithm::Brute,
        },
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(args) {
        Ok(cfg) => {
            let (result, grid) = execute(cfg);
            result.print_console();
            console::utils::print_puzzle(&grid);
        }
        Err(_) => println!("usage: generate <alg> <num iterations>"),
    }
}
