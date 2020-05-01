use rand::Rng;
use std::env;

//use std::io;
use sudoku::console::utils;
use sudoku::Cell;
use sudoku::Puzzle;

use std::collections::HashSet;

fn _check_peers(c: &Cell, grid: &[u8]) -> bool {
    let peers = c.peers();
    for i in peers {
        if grid[i] == c.value.unwrap() {
            let p = Cell::from_grid_idx(i as usize).with_value(grid[i]);
            println!("value exists: \n\tpeer: {}, \n\tupdate:{}", p, c);
            return false;
        }
    }
    true
}

fn candidates(c: &Cell, p: &Puzzle) -> Vec<u8> {
    // array of boolean. the index of the array denotes
    // the possible values and true/false indicates whether
    // the value is a possible value.
    let mut mapper: [bool; 9] = [false; 9];

    // get values from their peers and remove them
    // from candidates
    for p_idx in c.peers() {
        let v = p.grid_as_ref()[p_idx];

        if v != 0 {
            mapper[v as usize - 1] = true;
        }
    }

    let mut j: u8 = 0;

    let mut r: Vec<u8> = Vec::new();
    loop {
        if j >= 9 {
            break;
        }
        if mapper[j as usize] == false {
            r.push(j + 1)
        }
        j += 1;
    }
    r
}

fn guess(c: &Vec<u8>) -> Option<u8> {
    match c.len() {
        0 => None,
        1 => Some(c[0]),
        _ => {
            let v = rand::thread_rng().gen_range(0, c.len());
            Some(c[v])
        }
    }
}

fn generate(num_iterations: u32) {
    let mut my_board = Puzzle::new(&[0; 81]);
    let mut err_count = 0;
    let mut overwrites = 0;
    let mut existing: HashSet<usize> = HashSet::new();
    let mut invalid_vals = 0;

    for iter in 0..num_iterations {
        //let mut user_input = String::new();

        // pick the cell
        let idx: usize = rand::thread_rng().gen_range(0, 81);

        if existing.contains(&idx) {
            overwrites += 1;
            continue;
        } else {
            existing.insert(idx);
        }

        let c = Cell::from_grid_idx(idx);
        let cands = candidates(&c, &my_board);
        let v = guess(&cands);

        if v.is_none() {
            println!("no candidates for {}, candidates: {:?}", c, cands);
            break;
        }
        let c = c.with_value(v.unwrap());

        if _check_peers(&c, my_board.grid_as_ref()) == false {
            println!("candidates: {:?}", cands);
            invalid_vals += 1;
            continue;
        }
        match my_board.update_cell(&c) {
            Ok(_) => (),
            Err(_) => err_count += 1,
        };

        if my_board.is_completed() {
            break;
        }
        /*
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line");
        */
        println!("iter: {}, value: {}", iter, c);
        utils::print_console(&my_board);
    }
    let solved = my_board.is_completed();

    println!(
        "( iterations: {}, error_count: {}, %_error: {}, overwrite: {}, %_overwrites: {}, invalid_vals: {}, is_complete: {} )",
        num_iterations,
        err_count,
        (err_count as f64 / num_iterations as f64),
        overwrites,
        (overwrites as f64 / num_iterations as f64),
        invalid_vals,
        solved
    );
    my_board
        .grid_as_ref()
        .iter()
        .for_each(|x| print!("{} ", *x));
    println!("\nis solvable: {}", solvable(&my_board));
}

fn solvable(p: &Puzzle) -> bool {
    for i in 0..81 {
        let c = Cell::from_grid_idx(i);
        let candidates = candidates(&c, p);
        if candidates.len() == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };

    generate(num_iterations);
}
