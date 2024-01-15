
use crate::{get_puzzle, time_it};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper,
    Scissors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Result {
    Win = 6,
    Lose = 0,
    Draw = 3
}

impl RPS {
    fn from_string(input: &str) -> Self {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            _ => RPS::Scissors 
        }
    }

    fn invert(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            _ => RPS::Rock
        }
    }
}

impl Result {
    fn from_string(input: &str) -> Self {
        match input {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            _ => Result::Win
        }
    }
}

fn parse_pt1(input: &str) -> Vec<(RPS, RPS)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let game = l.split(" ")
                .map(|rps| RPS::from_string(rps))
                .collect::<Vec<RPS>>();
            (*&game[0], *&game[1])
        })
        .collect()
}

fn parse_pt2(input: &str) -> Vec<(RPS, Result)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let game: Vec<&str> = l.split(" ").collect();
            (RPS::from_string(&game[0]), Result::from_string(&game[1]) )
        })
        .collect()
}

fn solution_pt1(input: &str) -> u32 {
    let puzzle: Vec<(RPS, RPS)> = parse_pt1(input);
    let mut total: u32 = 0;

    for p in puzzle.iter() {
        let result: u32 = match p {
            (RPS::Rock, RPS::Scissors) => RPS::Scissors as u32,
            (RPS::Scissors, RPS::Paper) => RPS::Paper as u32,
            (RPS::Paper, RPS::Rock) => RPS::Rock as u32,
            (a, b) if a == b => 3 + (*b as u32),
            (_, b) => 6 + (*b as u32)
        };
        
        total += result;
    }

    total
}

fn solution_pt2(input: &str) -> u32 {
    let puzzle = parse_pt2(input);
    let mut total: u32 = 0;

    for p in puzzle.iter() {
        let result: u32 = match p {
            (rps, Result::Draw) => Result::Draw as u32 + *rps as u32,
            (RPS::Rock, r) => (*r as u32) + RPS::Rock.invert() as u32,
            (RPS::Paper, r) => (*r as u32) + RPS::Paper.invert() as u32,
            (RPS::Scissors, r) => (*r as u32) + RPS::Scissors.invert() as u32
        };
        println!("{:?} {}",p, result);
        total += result;
    }

    total
}

pub fn main() {
    let puzzle = get_puzzle("22", "2");

    time_it!("Part 1", solution_pt1(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &'static str = "\
A Y
B X
C Z
";

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_solution2() {
        let res = solution_pt2(TEST);
        assert_eq!(res, 12)
    }
}