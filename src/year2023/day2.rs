#![allow(unused)]

use regex::Regex;
use std::cmp::max;

use crate::{get_puzzle, time_it};

#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct SubGame(Colors, u32, u32);

struct Game {
    no: u32,
    games: Vec<SubGame>
}

fn parse(input: &str) -> Vec<Game> {
    let game = Regex::new(r"Game (\d+): (.+\n)").unwrap();
    let ginfo = Regex::new(r"\s*(\d+) (blue|green|red)(,|;|\n)").unwrap();
    // input.lines().collect()
    let mut res: Vec<Game> = vec![];
    for (_, [g_no, g]) in game.captures_iter(input).map(|c| c.extract()) {
        let mut games: Vec<SubGame> = vec![];
        let mut subg_no: u32 = 0;

        for (_, [num, col, sep]) in ginfo.captures_iter(g).map(|c| c.extract()) {

            let sg = match col {
                "blue" => SubGame(Colors::Blue, num.parse::<u32>().unwrap(), subg_no),
                "red" => SubGame(Colors::Red, num.parse::<u32>().unwrap(), subg_no),
                "green" => SubGame(Colors::Green, num.parse::<u32>().unwrap(), subg_no),
                _ => panic!()
            };

            if sep != "," {
                subg_no += 1;
            };

            games.push(sg)
        };
        let new_game = Game {
            no: g_no.parse::<u32>().unwrap(),
            games
        };
        res.push(new_game);
    }

    res
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Tracker {
    red: u32,
    green: u32,
    blue: u32,
}

impl Tracker {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn add(&mut self, sg: SubGame) {
        match (sg.0, sg.1) {
            (Colors::Red, s) => self.red = max(self.red, s),
            (Colors::Blue, s) => self.blue = max(self.blue, s),
            (Colors::Green, s) => self.green = max(self.green, s),
        }
    }

    fn clear(&mut self) {
        self.red = 0;
        self.blue = 0;
        self.green = 0;
    }

    fn check_valid(&self) -> bool {
        match self {
            s if self.red > MAX_RED => false,
            s if self.green > MAX_GREEN => false,
            s if self.blue > MAX_BLUE => false,
            _ => true
        }
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn solution_pt1(input: &str) -> u32 {
    let games = parse(input);
    let mut tracker = Tracker::new();

    let mut sum: u32 = 0;
    for game in games {
        let mut sg_no = game.games[0].2;
        let mut valid: bool = true;
        
        for sub in game.games {

            if sg_no != sub.2 {
                sg_no = sub.2;
                if !tracker.check_valid() {
                    
                    valid = false;
                    break
                }
                tracker.clear();
                
            }
            tracker.add(sub);

        };

        if valid && tracker.check_valid() {
            sum += game.no;
        }
        tracker.clear();
    }

    sum
}

fn solution_pt2(input: &str) -> u32 {
    let games = parse(input);
    let mut tracker = Tracker::new();
    let mut sum_power: u32 = 0;

    for game in games {
        for sub in game.games {
            tracker.add(sub);
        };
        sum_power += tracker.power();
        tracker.clear();
    }

    sum_power
}

pub fn main() {
    let data = get_puzzle("23", "2");

    time_it!("Part 1", solution_pt1(&data));
    time_it!("Part 2", solution_pt2(&data));
    // solution_pt2(&data);

}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_pt1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 8)
    }

    #[test]
    fn test_pt2() {
        let res = solution_pt2(TEST);
        assert_eq!(res, 2286)
    }

}