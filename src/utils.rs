use rand::Rng;
use sudoku::{Cell, Puzzle};

/// randomly pick a value from a slice of candidates. Return None if slice has length of zero.
pub fn pick<T: Copy>(c: &[T]) -> Option<T> {
    match c.len() {
        0 => None,
        1 => Some(c[0]),
        _ => {
            let v = rand::thread_rng().gen_range(0, c.len());
            Some(c[v])
        }
    }
}

/// Return a vector of possible values for a cell given the current state of the puzzle.
pub fn get_candidates(c: &Cell, p: &Puzzle) -> Vec<u8> {
    // array of boolean. the index of the array denotes
    // the possible values and true/false indicates whether
    // the value is a possible value.
    let mut mapper: [bool; 9] = [true; 9];

    // get values from their peers and remove them
    // from candidates
    for p_idx in c.peers() {
        let v = p.grid()[p_idx].value().unwrap_or(0);

        if v != 0 {
            mapper[v as usize - 1] = false;
        }
    }

    (0..9)
        .filter(|x| mapper[*x as usize])
        .map(|x| x + 1)
        .collect::<Vec<u8>>()
}

pub fn dump_puzzle(p: &Puzzle) {
    for cell in p.grid() {
        let val = cell.value().unwrap_or(0);
        print!("{} ", val);
    }
    println!("");
}

pub fn is_ok_with_peers(val: u8, peers: &[usize], p: &Puzzle) -> bool {
    let p_vals = p.grid();
    for p_idx in peers {
        let peer_cell = &p_vals[*p_idx];
        if peer_cell.value().is_some() {
            let peer_cell_value = peer_cell.value().unwrap_or(0);
            if peer_cell_value == val {
                return false;
            }
        }
    }
    true
}
