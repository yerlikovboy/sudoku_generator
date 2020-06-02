use sudoku_generator::gen::diag;

fn main() {
    println!("start");

    diag::generate(10000);
    println!("end");
}
