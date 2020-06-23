use std::env;

//use sudoku::console;
use sudoku_generator::cmd;
use sudoku_generator::cmd::{Algorithm, Config};
use sudoku_generator::gen::{brute, diag};
use sudoku_generator::job::result;

use sudoku_generator::console::ConsoleWriter;
use sudoku_generator::writer::ReportWriter;

fn execute(c: Config) -> result::Report {
    match c.algorithm() {
        Algorithm::Brute => brute::generate(&c),
        Algorithm::Diagonal => diag::generate(&c),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match cmd::parse_args(args) {
        Ok(cfg) => {
            let result = execute(cfg);
            ConsoleWriter::write(&result);
        }
        Err(_) => println!("usage: generate <alg> <num iterations>"),
    }
}
