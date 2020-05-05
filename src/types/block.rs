use sudoku::Cell;
use sudoku::Puzzle;

use crate::utils::candidates;

#[derive(Default, Debug, Clone)]
pub struct Block {
    pub _id: u8,
    _members: [usize; 9],
}

fn indices(x: usize, y: usize) -> [usize; 9] {
    let mut indexes: [usize; 9] = [0; 9];
    for i in 0..3 {
        let row = (x + i) * 9;
        for j in 0..3 {
            let col_off = y + j;
            indexes[(i * 3 + j) as usize] = row + col_off;
        }
    }
    indexes
}

impl Block {
    pub fn new(id: u8) -> Block {
        let (x, y) = match id {
            1 => (0, 0),
            2 => (0, 3),
            3 => (0, 6),
            4 => (3, 0),
            5 => (3, 3),
            6 => (3, 6),
            7 => (6, 0),
            8 => (6, 3),
            _ => (6, 6),
        };

        Block {
            _id: id,
            _members: indices(x, y),
        }
    }

    /// Returns true if cell is member of this block, false otherwise.
    pub fn is_member(&self, c: &Cell) -> bool {
        let cell_idx = c.as_grid_idx();
        self._members[..].iter().any(|x| *x == cell_idx)
    }

    // because we are passing in two references, out comes the lifetime specifier.
    pub fn from_cell(c: &Cell) -> Block {
        let x = (c.row as usize / 3) * 3;
        let y = (c.column as usize / 3) * 3;

        let id = match (x, y) {
            (0, 0) => 1,
            (0, 3) => 2,
            (0, 6) => 3,
            (3, 0) => 4,
            (3, 3) => 5,
            (3, 6) => 6,
            (_, 0) => 7,
            (_, 3) => 8,
            (_, _) => 9,
        };
        Block {
            _id: id,
            _members: indices(x, y),
        }
    }

    pub fn members(&self) -> &[usize] {
        &self._members[..]
    }

    /// returns an array of indices that correspond to block
    /// for which the cell is a member of excluding the index
    /// of the cell itself.
    pub fn members_for_cell(c: &Cell) -> [usize; 8] {
        let mut r: [usize; 8] = [0; 8];
        let x = (c.row as usize / 3) * 3;
        let y = (c.column as usize / 3) * 3;

        let idx_arr = indices(x, y);
        let c_idx = c.as_grid_idx();

        let mut r_idx: usize = 0;
        for i in idx_arr.iter() {
            if *i != c_idx {
                r[r_idx] = *i;
                r_idx += 1;
            }
        }
        r
    }

    /// Returns true if all the cells in the block
    /// have values and all values [1..9] are in the block
    /// or any all remaining values can be inserted into
    /// vacant cells.
    pub fn is_valid(&self, p: &Puzzle) -> bool {
        let mut vals: [bool; 10] = [false; 10];

        let mut blanks_idx: Vec<usize> = Vec::new();

        self._members.iter().for_each(|x| {
            let cur_val = p.grid()[*x].value().unwrap_or(0);
            if cur_val == 0 {
                blanks_idx.push(*x);
            }
            vals[cur_val as usize] = true;
        });

        if blanks_idx.len() == 0 {
            // confirm all numbers 1 thru 9 have been found or return false.
            return (1..10).all(|x| vals[x]);
        }

        for idx in blanks_idx {
            let c = Cell::from_grid_idx(idx);
            let cands = candidates::get_candidates(&c, p);
            // block can not be valid if there are no candidates for the empty cell
            if cands.len() == 0 {
                return false;
            }
            cands.iter().for_each(|x| vals[*x as usize] = true);
        }
        vals[..].iter().all(|x| *x == true)
    }
}
