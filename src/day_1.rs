use std::collections::HashMap;

// Left part A solution for posterity
fn solve_a(input: Vec<&str>) -> usize {
    let parse_calibration_value = |line: &str| -> usize {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digits
            .next()
            .expect("calibration string should contain a digit");
        let last_digit = digits.last().unwrap_or(first_digit);

        (first_digit * 10 + last_digit) as usize
    };

    input
        .into_iter()
        .map(|line| parse_calibration_value(line))
        .sum()
}

pub fn solve(input: Vec<&str>) -> usize {
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let parse_calibration_value = |line: &str| -> usize {
        let mut digits = Vec::<usize>::new();
        for i in 0..line.len() {
            for (key, value) in &map {
                if line[i..].starts_with(key) {
                    digits.push(*value);
                    break;
                }
            }
        }
        let first_digit = digits
            .first()
            .expect("calibration string should contain a digit");
        let last_digit = digits.last().unwrap_or(first_digit);

        first_digit * 10 + last_digit
    };

    input
        .into_iter()
        .map(|line| parse_calibration_value(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = solve(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_spelled_digits() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let result = solve(input);
        assert_eq!(result, 281);
    }
}
