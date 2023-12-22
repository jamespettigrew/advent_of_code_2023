use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand(String, bool); // bool is for part b Joker rule lmao

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    J, // Order changes in part b, breaks part a
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            _ => Card::Two
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let calc_hand_type = |h: &Self| {
            let mut counts = HashMap::<char, usize>::new();
            for c in h.0.chars() {
                counts.entry(c)
                    .and_modify(|count| { *count += 1})
                    .or_insert(1);
            }

            let hand_type = match counts.keys().len()  {
                1 => HandType::FiveOfAKind,
                2 => match *counts.values().max().unwrap() {
                    4 => HandType::FourOfAKind,
                    _ => HandType::FullHouse,
                },
                3 => {
                    let mut v = counts.values().map(|v| v.clone()).collect::<Vec<usize>>();
                    v.sort();
                    match &v[..] {
                        [1, 1, 3] => HandType::ThreeOfAKind,
                        _ => HandType::TwoPair,
                    }
                },
                4 => HandType::OnePair,
                _ => HandType::HighCard,
            };

            let mut joker_count = *counts.get(&'J').unwrap_or(&0);
            if !self.1 {
                joker_count = 0;
            }
            return match hand_type {
                HandType::FiveOfAKind => HandType::FiveOfAKind,
                HandType::FourOfAKind => match joker_count {
                    4 => HandType::FiveOfAKind,
                    1 => HandType::FiveOfAKind,
                    _ => HandType::FourOfAKind,
                },
                HandType::FullHouse => match joker_count {
                    3 => HandType::FiveOfAKind,
                    2 => HandType::FiveOfAKind,
                    _ => HandType::FullHouse,
                },
                HandType::ThreeOfAKind => match joker_count {
                    3 => HandType::FourOfAKind,
                    1 => HandType::FourOfAKind,
                    _ => HandType::ThreeOfAKind,
                },
                HandType::TwoPair => match joker_count {
                    2 => HandType::FourOfAKind,
                    1 => HandType::FullHouse,
                    _ => HandType::TwoPair,
                },
                HandType::OnePair => match joker_count {
                    2 => HandType::ThreeOfAKind,
                    1 => HandType::ThreeOfAKind,
                    _ => HandType::OnePair,
                },
                HandType::HighCard => match joker_count {
                    1 => HandType::OnePair,
                    _ => HandType::HighCard,
                },
            }
        };

        let lhs_type = calc_hand_type(self);
        let rhs_type = calc_hand_type(other);

        match HandType::cmp(&lhs_type, &rhs_type) {
            std::cmp::Ordering::Equal => {
                let lhs_cards = self.0.chars().map(|c| Card::from(c));
                let rhs_cards = other.0.chars().map(|c| Card::from(c));

                for (lhs, rhs) in lhs_cards.zip(rhs_cards) {
                    let o = Card::cmp(&lhs, &rhs);
                    if let std::cmp::Ordering::Equal = o {
                        continue
                    }

                    return o;
                }

                return std::cmp::Ordering::Equal;
            },
            x => x
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    let mut plays = Vec::<(Hand, usize)>::new();
    for play in input {
        let parts: Vec<&str> = play.split(" ").collect();
        let hand = Hand(parts[0].to_string(), false);
        let bid = parts[1].parse::<usize>().unwrap();
        plays.push((hand, bid));
    }

    plays.sort_by_key(|p| p.0.clone());
    let mut result = 0;
    for (i, play) in plays.into_iter().enumerate() {
        result += play.1 * (i + 1);
    }

    result
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let mut plays = Vec::<(Hand, usize)>::new();
    for play in input {
        let parts: Vec<&str> = play.split(" ").collect();
        let hand = Hand(parts[0].to_string(), true);
        let bid = parts[1].parse::<usize>().unwrap();
        plays.push((hand, bid));
    }

    plays.sort_by_key(|p| p.0.clone());
    let mut result = 0;
    for (i, play) in plays.into_iter().enumerate() {
        result += play.1 * (i + 1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483"
        ];
        assert_eq!(solve_a(&input), 6440);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
            "JQQQA 2000",
        ];
        assert_eq!(solve_b(&input), 13292);
    }
}
