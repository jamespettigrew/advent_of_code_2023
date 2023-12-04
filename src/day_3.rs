use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Character {
    Digit(u8),
    Dot,
    Symbol(String),
}

impl std::str::FromStr for Character {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = match s {
            "." => Character::Dot,
            potential_digit => match potential_digit.parse::<u8>() {
                Ok(d) => Character::Digit(d),
                _ => Character::Symbol(s.to_string()),
            },
        };

        Ok(char)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Schematic {
    characters: Vec<Character>,
    rows: usize,
    columns: usize,
}

impl From<&Vec<&str>> for Schematic {
    fn from(item: &Vec<&str>) -> Self {
        let mut characters = Vec::<Character>::new();
        let rows = item.len();
        let columns = item.first().unwrap_or(&"").len();
        for line in item {
            for i in 0..line.len() {
                let character = Character::from_str(&line[i..i + 1])
                    .expect("Character in schematic should be valid");
                characters.push(character);
            }
        }

        Schematic {
            characters,
            rows,
            columns,
        }
    }
}

impl Schematic {
    fn adjacencies(&self, idx: usize) -> Vec<usize> {
        let mut results = Vec::<usize>::new();
        let row = idx / self.columns;
        let column = idx % self.columns;

        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }

                let row = row as isize + r;
                let column = column as isize + c;
                if row >= 0
                    && row < self.rows as isize
                    && column >= 0
                    && column < self.columns as isize
                {
                    results.push(self.columns * row as usize + column as usize);
                }
            }
        }

        results
    }

    fn part_number_boundaries(&self, digit_idx: usize) -> (usize, usize) {
        let (mut west_boundary, mut east_boundary) = (digit_idx, digit_idx + 1);
        let column = digit_idx % self.columns;
        let (steps_west, steps_east) = (column, self.columns - column);

        for idx in (digit_idx + 1)..=(digit_idx + steps_east) {
            match self.characters[idx] {
                Character::Digit(_) => east_boundary = idx + 1,
                _ => break,
            };
        }

        for idx in ((digit_idx - steps_west)..digit_idx).rev() {
            match self.characters[idx] {
                Character::Digit(_) => west_boundary = idx,
                _ => break,
            };
        }

        (west_boundary, east_boundary)
    }

    fn part_number(&self, start: usize, end: usize) -> usize {
        let mut part_number = 0;
        for idx in start..end {
            if let Character::Digit(d) = self.characters[idx] {
                part_number *= 10;
                part_number += d as usize;
            }
        }

        part_number
    }
}

fn part_numbers(schematic: &Schematic) -> Vec<usize> {
    let adjacent_digit_coords = schematic
        .characters
        .iter()
        .enumerate()
        .filter(|(_, c)| matches!(c, Character::Symbol { .. }))
        .flat_map(|(idx, _)| schematic.adjacencies(idx))
        .filter(|idx| matches!(schematic.characters[*idx], Character::Digit { .. }));
    let unique_digit_coords: HashSet<usize> = HashSet::from_iter(adjacent_digit_coords);
    let part_number_boundaries = unique_digit_coords
        .into_iter()
        .map(|idx| schematic.part_number_boundaries(idx));
    let unique_boundaries: HashSet<(usize, usize)> = HashSet::from_iter(part_number_boundaries);
    unique_boundaries
        .into_iter()
        .map(|(start, end)| schematic.part_number(start, end))
        .collect()
}

fn gear_ratios(schematic: &Schematic) -> Vec<usize> {
    let symbol_adjacent_digit_coords = schematic
        .characters
        .iter()
        .enumerate()
        .filter(|(_, c)| match c {
            Character::Symbol(x) => x == "*",
            _ => false,
        })
        .map(|(idx, _)| {
            schematic
                .adjacencies(idx)
                .into_iter()
                .filter(|idx| matches!(schematic.characters[*idx], Character::Digit { .. }))
        });

    let mut result = Vec::<usize>::new();
    for adjacencies in symbol_adjacent_digit_coords {
        let unique_digit_coords: HashSet<usize> = HashSet::from_iter(adjacencies);
        let part_number_boundaries = unique_digit_coords
            .into_iter()
            .map(|idx| schematic.part_number_boundaries(idx));
        let unique_boundaries: HashSet<(usize, usize)> = HashSet::from_iter(part_number_boundaries);
        let part_numbers: Vec<usize> = unique_boundaries
            .into_iter()
            .map(|(start, end)| schematic.part_number(start, end))
            .collect();
        if part_numbers.len() == 2 {
            result.push(part_numbers[0] * part_numbers[1]);
        }
    }

    result
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    part_numbers(&Schematic::from(input)).iter().sum()
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    gear_ratios(&Schematic::from(input)).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schematic() {
        assert_eq!(
            Schematic::from(&vec!["12.*34", "#5..6$", "+789.."]),
            Schematic {
                characters: vec![
                    Character::Digit(1),
                    Character::Digit(2),
                    Character::Dot,
                    Character::Symbol("*".to_string()),
                    Character::Digit(3),
                    Character::Digit(4),
                    Character::Symbol("#".to_string()),
                    Character::Digit(5),
                    Character::Dot,
                    Character::Dot,
                    Character::Digit(6),
                    Character::Symbol("$".to_string()),
                    Character::Symbol("+".to_string()),
                    Character::Digit(7),
                    Character::Digit(8),
                    Character::Digit(9),
                    Character::Dot,
                    Character::Dot,
                ],
                rows: 3,
                columns: 6,
            }
        );
    }

    #[test]
    fn test_adjacencies() {
        let schematic = Schematic {
            characters: vec![Character::Dot; 25],
            rows: 5,
            columns: 5,
        };
        // North + West boundary checks
        assert_eq!(schematic.adjacencies(0), vec![1, 5, 6]);
        // North boundary check
        assert_eq!(schematic.adjacencies(1), vec![0, 2, 5, 6, 7]);
        // North + East boundary check
        assert_eq!(schematic.adjacencies(4), vec![3, 8, 9]);
        // No boundaries
        assert_eq!(schematic.adjacencies(8), vec![2, 3, 4, 7, 9, 12, 13, 14]);
        // South + West boundary checks
        assert_eq!(schematic.adjacencies(20), vec![15, 16, 21]);
        // South boundary checks
        assert_eq!(schematic.adjacencies(21), vec![15, 16, 17, 20, 22]);
        // South + East boundary checks
        assert_eq!(schematic.adjacencies(24), vec![18, 19, 23]);
        // West boundary checks
        assert_eq!(schematic.adjacencies(5), vec![0, 1, 6, 10, 11]);
        // East boundary checks
        assert_eq!(schematic.adjacencies(9), vec![3, 4, 8, 13, 14]);
    }

    #[test]
    fn test_part_number_boundaries() {
        let s = Schematic::from(&vec!["467..114..", "...*......", "..35..633."]);
        assert_eq!(s.part_number_boundaries(0), (0, 3));
        assert_eq!(s.part_number_boundaries(1), (0, 3));
        assert_eq!(s.part_number_boundaries(2), (0, 3));

        assert_eq!(s.part_number_boundaries(22), (22, 24));
        assert_eq!(s.part_number_boundaries(23), (22, 24));

        assert_eq!(s.part_number_boundaries(26), (26, 29));
        assert_eq!(s.part_number_boundaries(27), (26, 29));
        assert_eq!(s.part_number_boundaries(28), (26, 29));
    }

    #[test]
    fn test_part_numbers() {
        let s = Schematic::from(&vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);
        assert_eq!(
            part_numbers(&s).sort(),
            vec![467, 35, 633, 617, 592, 755, 664, 598].sort()
        );
    }

    #[test]
    fn test_solve_a() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        assert_eq!(solve_a(&input), 4361);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        assert_eq!(solve_b(&input), 467835);
    }
}
