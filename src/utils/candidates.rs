use sudoku::Cell;
use sudoku::Puzzle;

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
