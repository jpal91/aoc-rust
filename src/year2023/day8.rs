use std::{collections::HashMap, ops::Deref};

use regex::Regex;

use crate::{get_puzzle, time_it};

struct Part1<'lr>(&'lr str);

struct LeftRight<'lr> {
    lr: Vec<char>,
    map: HashMap<&'lr str, (&'lr str, &'lr str)>,
    index: usize,
    prev: &'lr str,
}

impl Deref for Part1<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'lr> Iterator for LeftRight<'lr> {
    type Item = &'lr str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.lr.len() {
            self.index = 0;
            // return None;
        }

        // println!("{}", self.prev);
        let next_idx = self.lr[self.index];
        let next_direct = self.map.get(self.prev).unwrap();
        let next = match next_idx {
            'L' => next_direct.0,
            _ => next_direct.1,
        };
        self.prev = next;
        self.index += 1;

        Some(next)
    }
}

fn parse(input: &str) -> LeftRight<'_> {
    let left_right_re = Regex::new(r"(L|R)+").unwrap();
    let locations_re =
        Regex::new(r"(?<start>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();

    let left_right = left_right_re.captures(input).unwrap()[0].chars().collect();

    let mut locations: HashMap<&str, (&str, &str)> = HashMap::new();

    for cap in locations_re.captures_iter(input) {
        let loc = cap.name("start").unwrap().as_str();
        let left = cap.name("left").unwrap().as_str();
        let right = cap.name("right").unwrap().as_str();

        locations.insert(loc, (left, right));
    }

    // println!("{:?}", left_right);
    // println!("{:?}", locations);

    LeftRight {
        lr: left_right,
        map: locations,
        index: 0,
        prev: "AAA",
    }
}

fn solution_pt1(input: &str) -> u32 {
    let left_right = parse(input);
    let mut count = 0;

    for direct in left_right {
        count += 1;

        if direct == "ZZZ" {
            break;
        }
    }

    count
}
pub fn main() {
    let puzzle = get_puzzle("23", "8");

    time_it!("Solution Pt 1", solution_pt1(&puzzle));
}

#[cfg(test)]
const TEST_STR: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
const TEST_STR2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let solution = solution_pt1(TEST_STR);
        let solution2 = solution_pt1(TEST_STR2);

        assert_eq!(solution, 2);
        assert_eq!(solution2, 6);
    }
}
