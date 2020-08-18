use std::{env, fs};

use sudoku_generator;
use sudoku_generator::types::{generated, result};
use sudoku_generator::utils;

fn make_puzzle(grid: &Vec<u8>, clue_count: u8, map_id: String) -> generated::Puzzle {
    let mut puzzle: [u8; 81] = [0; 81];
    let idx_vals: Vec<u8> = (0..81).collect();

    let mut count = 0;

    while count < clue_count {
        let idx = utils::pick(&idx_vals).unwrap() as usize;

        if puzzle[idx] == 0 {
            puzzle[idx] = grid[idx];
            count += 1;
        }
    }

    generated::Puzzle::new(map_id.as_str(), clue_count, &puzzle[..])
}

struct Config {
    pub fname: Option<String>,
    pub num_clues: u8,
}

// Options
//
//  -f filename. Send in result with puzzle_complete set to true.
//  -c number of clues
//
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: spd -f <filename> -c <num_clues>");
        return;
    }

    let mut iter = args.iter();
    let mut cfg = Config {
        fname: None,
        num_clues: 25,
    };

    iter.next();

    while let Some(opt) = iter.next() {
        match opt.as_str() {
            "-f" => {
                if let Some(fname) = iter.next() {
                    cfg.fname = Some(String::from(fname));
                } else {
                    println!("missing filename");
                    return;
                }
            }
            "-c" => {
                if let Some(n_clues) = iter.next() {
                    let n = n_clues.parse::<u8>().unwrap();

                    if n > 65 {
                        println!("number of clues must be less than 65");
                        return;
                    }
                    cfg.num_clues = n_clues.parse::<u8>().unwrap();
                } else {
                    println!("missing number of clues");
                    return;
                }
            }
            _ => {
                println!("unknown option");
                return;
            }
        }
    }

    let report = fs::read_to_string(cfg.fname.unwrap()).expect("unable to read from file");
    let d: result::Report = serde_json::from_str(&report).unwrap();
    let p = make_puzzle(d.grid().unwrap(), cfg.num_clues, d.id());

    // p.grid()
    //     .iter()
    //     .for_each(|x| print!("{} ", x.value().unwrap_or(0)));
    // println!("");
    p.dump_console();
}
