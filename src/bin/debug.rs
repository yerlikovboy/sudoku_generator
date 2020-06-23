use sudoku::Puzzle;
use sudoku_generator::types::block;

use sudoku_generator::cmd::{Algorithm, Config};
use sudoku_generator::job::result::Report;
fn test() {
    println!("test");

    let r = Report::new(&Config::new(10, Algorithm::Brute));

    let r_json = serde_json::to_string(&r).unwrap();
    println!("this is r: {:?}", r);
    println!("as json: {}", r_json);

    println!("test finished: {:?}", r);
}
fn main() {
    println!("start debug of Block::is_valid");

    let grid: [u8; 81] = [
        6, 1, 8, 4, 5, 9, 7, 2, 3, 7, 4, 3, 6, 2, 8, 5, 1, 9, 5, 2, 9, 1, 3, 7, 8, 6, 4, 1, 5, 2,
        7, 4, 6, 9, 3, 8, 3, 6, 4, 8, 9, 5, 2, 7, 1, 9, 8, 7, 2, 1, 3, 6, 4, 5, 2, 3, 5, 9, 6, 4,
        1, 8, 7, 4, 7, 6, 5, 8, 1, 3, 9, 2, 8, 9, 1, 3, 7, 2, 4, 5, 6,
    ];

    let p = Puzzle::new(&grid[..]);

    let mut blocks: Vec<block::Block> = Vec::new();
    (1..10).for_each(|x| blocks.push(block::Block::new(x)));

    for i in 0..9 {
        // b cannot take ownership (aka blocks[i] cannot move to b) so we must use reference
        let b = &blocks[i];
        println!("block id: {}, members:{:?}", b._id, b.members());
        println!("is_valid: {}", b.is_valid(&p));
    }
    println!("all is_valid: {}", blocks.iter().all(|x| x.is_valid(&p)));

    println!("calling test");

    test();

    println!("Finished");
}
