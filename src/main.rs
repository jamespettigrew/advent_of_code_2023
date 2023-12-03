mod day_1;
mod day_2;

use std::fs;

fn main() {
    let contents = fs::read_to_string("day_1a.in").expect("Day 1a input file should be present");
    let input_a = contents.lines().collect();
    println!("Day 1: {}", day_1::solve(input_a));

    let contents = fs::read_to_string("day_2a.in").expect("Day 2a input file should be present");
    let input_a = contents.lines().collect();
    println!(
        "Day 2: {} | {}",
        day_2::solve_a(&input_a),
        day_2::solve_b(&input_a)
    );
}
