use std::io::{stdin, BufRead};

mod helpers;
mod puzzles;

fn main() {
    println!("Advent of Code 2024\n");
    print!("Puzzle to run : ");
    let puzzle: String = text_io::read!();
    println!();

    puzzles::solve(&puzzle);
}
