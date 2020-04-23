use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

use rand::Rng;
use sudoku::Board;
use sudoku::Cell;

use std::collections::HashSet;
fn _fluent_read(file_name: &str) -> io::Result<Vec<u8>> {
    println!("fluent read");
    let f = File::open(file_name)?;

    let vals: Vec<u8> = f
        .bytes()
        .map(|x| x.unwrap())
        .filter(|x| x.is_ascii_digit())
        .map(|x| x - 48)
        .collect();

    println!("# values read: {}", vals.len());
    Ok(vals)
}

fn try_sudoku(num_iterations: u32) {
    let mut my_board = Board::init();
    let mut err_count = 0;
    let mut overwrites = 0;
    let mut existing: HashSet<u8> = HashSet::new();

    for _ in 0..num_iterations {
        let v = rand::thread_rng().gen_range(1, 10);

        let idx: u8 = rand::thread_rng().gen_range(0, 81);

        if existing.contains(&idx) {
            overwrites += 1;
        } else {
            existing.insert(idx);
        }

        let x = idx / 9;
        let y = idx % 9;

        match Cell::new(x + 1, y + 1, v) {
            Ok(v) => match my_board.update_cell(&v) {
                Ok(_) => (),
                Err(_) => err_count += 1,
            },
            Err(_) => (),
        }
    }
    let solved = my_board.is_completed();
    println!(
        "( iterations: {}, error_count: {}, %_error: {}, overwrite: {}, %_overwrites: {}, is_complete: {} )",
        num_iterations,
        err_count,
        (err_count as f64 / num_iterations as f64),
        overwrites,
        (overwrites as f64 / num_iterations as f64),
        solved
    );
    if solved {
        my_board.grid().iter().for_each(|x| print!("{} ", *x));
        println!("");
    }
    // my_board.print_console();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //let fname = "/Users/ali/dev/rust/projects/sudoku/board.txt";
    //println!("{:?}", fluent_read(fname).unwrap());

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };

    try_sudoku(num_iterations);
}
