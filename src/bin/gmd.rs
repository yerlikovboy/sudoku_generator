use std::{env, thread, time};

//use sudoku::console;
use sudoku_generator::cmd;
use sudoku_generator::cmd::{Algorithm, Config};
use sudoku_generator::gen::{brute, diag};
use sudoku_generator::job::result;

use sudoku_generator::console::ConsoleWriter;
use sudoku_generator::writer::ReportWriter;

fn execute(c: &Config) -> result::Report {
    match c.algorithm() {
        Algorithm::Brute => brute::generate(&c),
        Algorithm::Diagonal => diag::generate(&c),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ten_secs = time::Duration::from_secs(10);
    match cmd::parse_args(args) {
        Ok(cfg) => loop {
            let result = execute(&cfg);
            ConsoleWriter::write(&result);
            thread::sleep(ten_secs);
        },
        Err(_) => println!("usage: generate <alg> <num iterations>"),
    }
}
