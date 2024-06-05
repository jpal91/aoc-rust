#![allow(unused)]
use regex::Regex;
use std::ops::Range;

use crate::{get_puzzle, time_it};

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let re = Regex::new(r"(\d+)+").unwrap();
    let mut lines = input.lines();

    let time: Vec<u32> = re
        .find_iter(lines.next().unwrap())
        .map(|c| c.as_str().parse::<u32>().unwrap())
        .collect();
    let distance: Vec<u32> = re
        .find_iter(lines.next().unwrap())
        .map(|c| c.as_str().parse::<u32>().unwrap())
        .collect();

    (time, distance)
}

fn parse_pt2(input: &str) -> (u64, u64) {
    let re = Regex::new(r"(\d+)+").unwrap();
    let mut lines = input.lines();

    let time: u64 = re
        .find_iter(lines.next().unwrap())
        .map(|c| c.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let distance: u64 = re
        .find_iter(lines.next().unwrap())
        .map(|c| c.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    (time, distance)
}

fn solution_pt1(input: &str) -> u32 {
    let (time, distance): (Vec<u32>, Vec<u32>) = parse(input);

    let res: u32 = time
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| {
            (0..t + 1)
                .map(|time: u32| (t - time) * time)
                .filter(|result| result > d)
                .collect::<Vec<u32>>()
                .len() as u32
        })
        .product();

    res
}

fn get_bounds(time: &u64, distance: &u64, reversed: bool) -> u64 {
    if reversed {
        (0..*time)
            .rev()
            .map(|t| (time - t) * t)
            .take_while(|r| r < distance)
            .collect::<Vec<u64>>()
            .len() as u64
    } else {
        (0..*time)
            .map(|t| (time - t) * t)
            .take_while(|r| r < distance)
            .collect::<Vec<u64>>()
            .len() as u64
    }
}

fn solution_pt2(input: &str) -> u64 {
    let (time, distance): (u64, u64) = parse_pt2(input);

    let upper = get_bounds(&time, &distance, true);
    let lower = get_bounds(&time, &distance, false);

    time - (upper + lower)
}

pub fn main() {
    let puzzle = get_puzzle("23", "6");

    time_it!("Part 1", solution_pt1(&puzzle));
    time_it!("Part 2", solution_pt2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 288)
    }

    #[test]
    fn test_solution2() {
        let res = solution_pt2(TEST);
        assert_eq!(res, 71503)
    }
}

