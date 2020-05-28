use std::collections::HashSet;
use std::str;

// for now, we only do classic 9x9 sudoku
pub struct Grid {
    _grid: Vec<u8>,
    _clues_indices: HashSet<usize>,
}

fn peers(idx: usize) -> Vec<usize> {
    let mut _mapper: [bool; 81] = [false; 81];

    //rows
    let row_idx: usize = (idx / 9) * 9;
    let row_end: usize = row_idx + 9;
    (row_idx..row_end)
        .filter(|x| *x != idx)
        .for_each(|x| _mapper[x] = true);

    // cols
    let col_offset: usize = idx % 9;
    (0..9)
        .map(|x| (x as usize * 9) + col_offset)
        .filter(|x| *x != idx)
        .for_each(|x| _mapper[x] = true);

    // block
    let block_x = ((idx / 9) / 3) * 3;
    let block_y = ((idx % 9) / 3) * 3;

    for i in 0..3 {
        let row_offset = (block_x + i) * 9;
        for j in 0..3 {
            let col_idx = block_y + j;
            let idx = row_offset + col_idx;
            if idx != idx {
                _mapper[idx] = true;
            }
        }
    }

    (0..81)
        .filter(|x| _mapper[*x as usize] == true)
        .collect::<Vec<usize>>()
}

impl Grid {
    pub fn new(grid: &[u8]) -> Grid {
        Grid {
            _grid: grid.to_vec(),
            _clues_indices: (0..81)
                .filter(|x| grid[*x as usize] != 0)
                .collect::<HashSet<usize>>(),
        }
    }

    pub fn grid_as_ref(&self) -> &[u8] {
        &self._grid
    }

    fn check_peers(&self, val: u8, idx: usize) -> bool {
        let peers = peers(idx);
        for idx in peers {
            if self._grid[idx] == val {
                return true;
            }
        }
        false
    }

    pub fn update_cell(&mut self, val: u8, idx: usize) -> Result<(), &str> {
        if self._clues_indices.contains(&idx) {
            return Err("cannot update initial board value");
        }

        if val != 0 && self.check_peers(val, idx) {
            return Err("cell has a peer which already contains value");
        }

        self._grid[idx] = val;
        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        for i in 0..81 {
            if self._grid[i] == 0 {
                return false;
            }
        }
        true
    }
}
