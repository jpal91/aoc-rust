use std::{collections::HashMap, hash::Hash, ops::Deref};

use num_integer::Integer;
use regex::Regex;

use crate::{get_puzzle, time_it};

#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
struct Part2<'lr>(&'lr str, &'lr str);

#[derive(Debug)]
struct LeftRight<T> {
    lr: Vec<char>,
    map: HashMap<T, (T, T)>,
    index: usize,
    prev: T,
    part2: Vec<T>,
}

impl LeftRight<Part2<'_>> {
    fn add_part2(&mut self) {
        self.map.keys().for_each(|key| {
            if key.is_a() {
                self.part2.push(key.clone());
            }
        });
    }
}

impl Part2<'_> {
    fn is_a(&self) -> bool {
        self.1 == "A"
    }

    fn is_z(&self) -> bool {
        self.1 == "Z"
    }
}

impl<'lr> From<&'lr str> for Part2<'lr> {
    fn from(value: &'lr str) -> Self {
        let num = &value[..2];
        let end = &value[2..];
        Part2(num, end)
    }
}

impl<'lr> Iterator for LeftRight<&'lr str> {
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

fn parse<'lr, T>(input: &'lr str) -> LeftRight<T>
where
    T: From<&'lr str> + PartialEq + Eq + Hash + Default,
{
    let left_right_re = Regex::new(r"(L|R)+").unwrap();
    let locations_re =
        Regex::new(r"(?<start>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
            .unwrap();

    let left_right = left_right_re.captures(input).unwrap()[0].chars().collect();

    let mut locations: HashMap<T, (T, T)> = HashMap::new();

    for cap in locations_re.captures_iter(input) {
        let loc = cap.name("start").unwrap().as_str();
        let left = cap.name("left").unwrap().as_str();
        let right = cap.name("right").unwrap().as_str();

        locations.insert(loc.into(), (left.into(), right.into()));
    }

    LeftRight {
        lr: left_right,
        map: locations,
        index: 0,
        prev: T::default(),
        part2: vec![],
    }
}

fn solution_pt1(input: &str) -> u32 {
    let mut left_right = parse(input);
    left_right.prev = "AAA";
    let mut count = 0;

    for direct in left_right {
        count += 1;

        if direct == "ZZZ" {
            break;
        }
    }

    count
}

fn solution_pt2(input: &str) -> u64 {
    let mut left_right = parse(input);
    left_right.add_part2();

    let mut counts = vec![];
    let n = left_right.lr.len();

    for key in left_right.part2.iter() {
        let mut idx = 0;
        let mut current = key.clone();
        let mut count = 0;

        while !current.is_z() {
            let item = left_right.map.get(&current).unwrap();

            current = match left_right.lr[idx] {
                'L' => item.0.clone(),
                _ => item.1.clone(),
            };
            count += 1;
            idx = (idx + 1) % n;
        }
        counts.push(count);
    }

    counts.iter().fold(1, |acc, x| acc.lcm(x))
}

pub fn main() {
    let puzzle = get_puzzle("23", "8");

    time_it!("Solution Pt 1", solution_pt1(&puzzle));
    time_it!("Solution Pt 2", solution_pt2(&puzzle));
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
const TEST_STR3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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

    #[test]
    fn test_pt2() {
        let solution = solution_pt2(TEST_STR3);

        assert_eq!(solution, 6)
    }
}
