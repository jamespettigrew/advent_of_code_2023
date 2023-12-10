fn ways(total_time: usize, distance_record: usize) -> Vec<usize> {
    let mut results = Vec::<usize>::new();
    for time_held in 1..total_time {
        let distance = time_held * (total_time - time_held);
        if distance > distance_record {
            results.push(time_held);
        }
    }

    results
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    let times = input[0]
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());
    let distances = input[1]
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());

    times
        .zip(distances)
        .map(|record| ways(record.0, record.1))
        .map(|ways| ways.len())
        .product()
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let time = input[0]
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok())
        .fold("".to_string(), |acc, i| acc + &i.to_string())
        .parse::<usize>()
        .unwrap();
    let distance = input[1]
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .fold("".to_string(), |acc, i| acc + &i.to_string())
        .parse::<usize>()
        .unwrap();

    ways(time, distance).iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        assert_eq!(solve_a(&input), 288);
    }

    #[test]
    fn test_solve_b() {
        let input = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        assert_eq!(solve_b(&input), 71503);
    }
}
