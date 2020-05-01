use std::env;
use sudoku::Cell;
use sudoku::Puzzle;

fn gen(n: u32) {
    println!("# iterations: {}", n);
}

fn _candidates(c: &Cell, p: &Puzzle) -> Vec<u8> {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_iterations: u32 = if args.len() > 1 {
        *&args[1].parse::<u32>().unwrap()
    } else {
        81
    };
    gen(num_iterations);
}
