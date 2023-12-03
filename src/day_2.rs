use std::cmp;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum CubeColour {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl std::str::FromStr for CubeColour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColour::Red),
            "blue" => Ok(CubeColour::Blue),
            "green" => Ok(CubeColour::Green),
            _ => Err(format!("'{}' is not a valid value for CubeColour", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game(usize);

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = Err(format!("'{}' is not a valid Game ID string", s));
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 || parts[0] != "Game" {
            return err;
        }

        match parts[1].parse::<usize>() {
            Ok(id) => Ok(Game(id)),
            Err(_) => err,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Handful([usize; 3]);

impl std::str::FromStr for Handful {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = format!("'{}' is not a valid Handful", s);

        let mut cubes = [0; 3];
        for p in s.trim().split(",") {
            let parts: Vec<&str> = p.trim().split(" ").collect();
            if parts.len() != 2 {
                return Err(err_msg);
            }

            let quantity = parts[0].parse::<usize>().map_err(|_| err_msg.clone())?;
            let colour = CubeColour::from_str(parts[1]).map_err(|_| err_msg.clone())?;

            cubes[colour as usize] += quantity;
        }

        Ok(Handful(cubes))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GameRecord(Game, Vec<Handful>);

impl std::str::FromStr for GameRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = format!("'{}' is not a valid GameRecord", s);

        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(err_msg);
        }

        let game = Game::from_str(parts[0]).map_err(|_| err_msg.clone())?;
        let handful_parts: Vec<&str> = parts[1].trim().split(";").collect();

        let mut handfuls = vec![];
        for p in handful_parts {
            let handful = Handful::from_str(p).map_err(|_| err_msg.clone())?;
            handfuls.push(handful);
        }

        Ok(GameRecord(game, handfuls))
    }
}

impl GameRecord {
    fn possible_game(&self) -> bool {
        for handful in self.1.iter() {
            let mut bag = [0; 3];
            bag[CubeColour::Red as usize] = 12;
            bag[CubeColour::Green as usize] = 13;
            bag[CubeColour::Blue as usize] = 14;

            for colour_idx in 0..3 {
                if handful.0[colour_idx] > bag[colour_idx] {
                    return false;
                }
            }
        }

        true
    }

    fn cubes_needed(&self) -> [usize; 3] {
        let mut max = [0; 3];
        for handful in self.1.iter() {
            for colour_idx in 0..3 {
                max[colour_idx] = cmp::max(max[colour_idx], handful.0[colour_idx]);
            }
        }

        max
    }
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| GameRecord::from_str(line))
        .collect::<Result<Vec<GameRecord>, _>>()
        .expect("All GameRecord's should be valid")
        .iter()
        .filter(|r| r.possible_game())
        .map(|r| r.0 .0)
        .sum()
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| GameRecord::from_str(line))
        .collect::<Result<Vec<GameRecord>, _>>()
        .expect("All GameRecord's should be valid")
        .iter()
        .map(|r| r.cubes_needed())
        .map(|cubes_needed| cubes_needed.into_iter().fold(1, |sum, i| sum * i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_record() {
        assert_eq!(
            GameRecord::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
            GameRecord(Game(5), vec![Handful([6, 3, 1]), Handful([1, 2, 2])])
        );
        assert_eq!(
            GameRecord::from_str("Game 100: 6 red, 1 blue, 3 green, 26 red, 4 blue; 2 blue, 13 green, 1 red, 2 green").unwrap(),
            GameRecord(Game(100), vec![Handful([32, 3, 5]), Handful([1, 15, 2])])
        );
    }

    #[test]
    fn test_possible_game() {
        assert_eq!(GameRecord(Game(0), vec![]).possible_game(), true);
        assert_eq!(
            GameRecord(Game(0), vec![Handful([0, 0, 0])]).possible_game(),
            true
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([12, 0, 0])]).possible_game(),
            true
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([0, 13, 0])]).possible_game(),
            true
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([0, 0, 14])]).possible_game(),
            true
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([12, 13, 14])]).possible_game(),
            true
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([13, 0, 0])]).possible_game(),
            false
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([0, 14, 0])]).possible_game(),
            false
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([0, 0, 15])]).possible_game(),
            false
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([13, 14, 15])]).possible_game(),
            false
        );
    }

    #[test]
    fn test_cubes_needed() {
        assert_eq!(
            GameRecord(Game(0), vec![Handful([13, 14, 15])]).cubes_needed(),
            [13, 14, 15]
        );
        assert_eq!(
            GameRecord(Game(0), vec![Handful([13, 14, 15]), Handful([28, 11, 3])]).cubes_needed(),
            [28, 14, 15]
        );
    }

    #[test]
    fn test_solve_a() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let result = solve_a(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let result = solve_b(&input);

        assert_eq!(result, 2286);
    }
}
