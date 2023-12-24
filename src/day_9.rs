pub fn solve_a(input: &Vec<&str>) -> usize {
    let mut sum = 0;

    fn seq_next(digits: &[isize]) -> isize {
        if digits.iter().all(|d| *d == 0) {
            return 0
        }

        let last = digits.last().expect("Digits should be non empty.");
        let deltas: Vec<isize> = digits.windows(2).map(|w| w[1] - w[0]).collect();

        last + seq_next(&deltas)
    }

    for history in input {
        let digits: Vec<isize> = history.split_whitespace().filter_map(|s| s.parse::<isize>().ok()).collect();
        sum += seq_next(&digits);
    }

    sum as usize
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let mut sum = 0;

    fn seq_prev(digits: &[isize]) -> isize {
        if digits.iter().all(|d| *d == 0) {
            return 0
        }

        let deltas: Vec<isize> = digits.windows(2).map(|w| w[1] - w[0]).collect();

        digits[0] - seq_prev(&deltas)
    }

    for history in input {
        let digits: Vec<isize> = history.split_whitespace().filter_map(|s| s.parse::<isize>().ok()).collect();
        sum += seq_prev(&digits);
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec![
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ];
        assert_eq!(solve_a(&input), 114);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ];
        assert_eq!(solve_b(&input), 2);
    }
}
