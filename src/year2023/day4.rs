#![allow(unused)]
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{get_puzzle, time_it};

fn parse(input: &str) -> Vec<CardGame> {
    let card_game = Regex::new(r"Card\s*(\d+):\s*(.+) \|\s*(.+)\n").unwrap();
    let split_cards = Regex::new(r"\s+").unwrap();

    let mut res: Vec<CardGame> = vec![];

    for (_, [g_num, win, has]) in card_game.captures_iter(input).map(|c| c.extract()) {
        let win_cards: HashSet<u32> = split_cards
            .split(win)
            .map(|w| w.trim().parse::<u32>().unwrap())
            .collect();
        let have_cards: HashSet<u32> = split_cards
            .split(has)
            .map(|w| w.trim().parse::<u32>().unwrap())
            .collect();

        let new_game = CardGame {
            no: g_num.parse::<u32>().unwrap(),
            win: win_cards,
            has: have_cards,
        };

        res.push(new_game)
    }

    res
}

#[derive(Debug)]
struct CardGame {
    no: u32,
    win: HashSet<u32>,
    has: HashSet<u32>,
}

#[derive(Debug, Clone)]
struct GameResult {
    no: u32,
    count: u32,
    matches: u32,
}

impl CardGame {
    fn new(no: u32) -> Self {
        Self {
            no,
            win: HashSet::new(),
            has: HashSet::new(),
        }
    }

    fn get_wins(&self) -> Option<Vec<u32>> {
        let res: Vec<u32> = self
            .win
            .intersection(&self.has)
            .map(|v| v.to_owned())
            .collect();

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    fn get_points_pt1(&self) -> u32 {
        if let Some(res) = self.get_wins() {
            res.iter().skip(1).fold(1, |acc, _| acc * 2)
        } else {
            0
        }
    }

    fn get_points_pt2(&self) -> GameResult {
        if let Some(res) = self.get_wins() {
            GameResult {
                no: self.no - 1,
                count: 1,
                matches: res.len().try_into().unwrap(),
            }
        } else {
            GameResult {
                no: self.no - 1,
                count: 1,
                matches: 0,
            }
        }
    }
}

impl Iterator for GameResult {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.matches > 0 {
            let res = Some(self.matches + self.no);
            self.matches -= 1;
            res
        } else {
            None
        }
    }
}

fn solution_pt1(input: &str) -> u32 {
    let games = parse(input);
    games.iter().map(|cg: &CardGame| cg.get_points_pt1()).sum()
}

fn solution_pt2(input: &str) -> u32 {
    let mut results: Vec<GameResult> = parse(input)
        .iter()
        .map(|c: &CardGame| c.get_points_pt2())
        .collect();
    let mut total: u32 = 0;
    let n: usize = results.len();

    for i in 0..n {
        let mut result: GameResult = results.get(i).unwrap().clone();
        total += result.count;

        while let Some(mut r) = result.next() {
            let next_idx: usize = (r as usize);
            if next_idx >= n {
                continue;
            }
            results[next_idx].count += result.count;
        }
    }

    total
}

pub fn main() {
    let puzzle = get_puzzle("23", "4");

    time_it!("Part 1", solution_pt1(&puzzle));
    time_it!("Part 2", solution_pt2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_parse() {
        parse(TEST);
    }

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 13)
    }

    #[test]
    fn test_solution2() {
        let res = solution_pt2(TEST);
        assert_eq!(res, 30)
    }
}

