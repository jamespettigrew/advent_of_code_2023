mod day_1;

use std::fs;

fn main() {
    let contents = fs::read_to_string("day_1a.in").expect("Day 1a input file should be present");
    let input_a = contents.lines().collect();

    println!("Day 1: {}", day_1::solve(input_a));
}
