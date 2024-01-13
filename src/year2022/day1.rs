#![allow(unused)]

use crate::{get_puzzle, time_it};

fn solution_pt1(input: &str) -> i32 {
    let mut calories = input
        .split("\n\n")
        .map(|line: &str| line.split("\n").map(|num: &str| num.parse::<i32>().unwrap_or_default()).sum())
        .collect::<Vec<i32>>();

    calories.sort_by_key(|n| -1 * n);

    *&calories[0]

}

pub fn main() {
    let puzzle = get_puzzle("22", "1");

    time_it!("Part 1", solution_pt1(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &'static str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 24000)
    }
}