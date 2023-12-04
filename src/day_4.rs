use std::collections::{HashSet, VecDeque};

struct Scratchcard(usize);

impl From<&str> for Scratchcard {
    fn from(record: &str) -> Self {
        let card: Vec<&str> = record.split(":").collect();
        let sides: Vec<&str> = card[1].split(" | ").collect();
        let winning_numbers = sides[0]
            .trim()
            .split(" ")
            .into_iter()
            .filter_map(|s| s.parse::<usize>().ok());
        let winning_numbers = HashSet::<usize>::from_iter(winning_numbers);
        let my_numbers: Vec<usize> = sides[1]
            .trim()
            .split(" ")
            .into_iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        let mut matches = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        Scratchcard(matches)
    }
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    let mut total_points = 0;
    for record in input {
        let scratchcard = Scratchcard::from(*record);
        let matches = scratchcard.0;
        if matches > 0 {
            total_points += 2usize.pow((matches - 1) as u32);
        }
    }

    total_points
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let scratchcards: Vec<Scratchcard> = input
        .into_iter()
        .map(|record| Scratchcard::from(*record))
        .collect();

    let mut scratchcard_queue = VecDeque::from_iter(scratchcards.iter().enumerate());
    let mut total = scratchcard_queue.len();
    while let Some((idx, scratchcard)) = scratchcard_queue.pop_front() {
        let matches = scratchcard.0;
        let start = idx + 1;
        let end = idx + 1 + matches;
        for i in start..end {
            scratchcard_queue.push_back((i, &scratchcards[i]));
        }
        total += matches;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        assert_eq!(solve_a(&input), 13);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        assert_eq!(solve_b(&input), 30);
    }
}
