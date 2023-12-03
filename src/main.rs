mod day_1;
mod day_2;
mod day_3;

use std::fs;

struct Day {
    input_a: String,
    input_b: String,
    solve_a: fn(&Vec<&str>) -> usize,
    solve_b: fn(&Vec<&str>) -> usize,
}

fn main() {
    let days = vec![
        Day {
            input_a: "day_1a.in".to_string(),
            input_b: "day_1a.in".to_string(),
            solve_a: day_1::solve_a,
            solve_b: day_1::solve_b,
        },
        Day {
            input_a: "day_2a.in".to_string(),
            input_b: "day_2a.in".to_string(),
            solve_a: day_2::solve_a,
            solve_b: day_2::solve_b,
        },
        Day {
            input_a: "day_3a.in".to_string(),
            input_b: "day_3a.in".to_string(),
            solve_a: day_3::solve_a,
            solve_b: day_3::solve_b,
        },
    ];

    for (idx, day) in days.iter().enumerate() {
        let a_contents = fs::read_to_string(&day.input_a)
            .expect(&format!("Day {}a input file should be present", idx + 1));
        let input_a = a_contents.lines().collect();
        let b_contents = fs::read_to_string(&day.input_b)
            .expect(&format!("Day {}b input file should be present", idx + 1));
        let input_b = b_contents.lines().collect();
        println!(
            "Day {}: {} | {}",
            idx + 1,
            (day.solve_a)(&input_a),
            (day.solve_b)(&input_b)
        );
    }
}
