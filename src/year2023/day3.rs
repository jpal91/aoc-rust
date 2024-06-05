#![allow(unused)]
use std::{collections::HashSet, ops::Deref};

use crate::{get_puzzle, time_it};
use ::grid::prelude::*;

fn check_adjacent(
    cell: &Cell<char, ()>,
    grid: &DefaultGrid<char>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let mut number: Vec<char> = vec![*cell.to_owned()];
    let (mut ny, mut nx) = cell.coords();
    let start = nx;

    while let Some(cell) = grid.get_neighbor((ny, nx), 6) {
        if cell.is_numeric() {
            number.insert(0, *cell.to_owned());
            visited.insert(cell.coords());
            nx -= 1;
        } else {
            break;
        }
    }

    nx = start;

    while let Some(cell) = grid.get_neighbor((ny, nx), 2) {
        if cell.is_numeric() {
            number.push(*cell.to_owned());
            visited.insert(cell.coords());
            nx += 1;
        } else {
            break;
        }
    }

    number.iter().collect::<String>().parse::<i32>().unwrap()
}

fn find_numbers(
    cell: &Cell<char, ()>,
    grid: &DefaultGrid<char>,
    visited: &mut HashSet<(usize, usize)>,
    part2: bool,
) -> Option<Vec<i32>> {
    let mut res: Vec<i32> = vec![];

    for cell in grid.neighbors(cell) {
        let coords = cell.coords();
        if visited.contains(&coords) {
            continue;
        };

        if cell.val.is_numeric() {
            visited.insert(coords);
            let adj: i32 = check_adjacent(cell, grid, visited);
            res.push(adj)
        }
    }
    // res
    match (part2, res.len()) {
        (true, 2) => Some(res),
        (false, _) => Some(res),
        _ => None,
    }
}
//
fn solution_pt1(grid: &DefaultGrid<char>) -> i32 {
    let mut nums: Vec<i32> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, x, item) in grid.iter_enum() {
        if **item == '.' || item.is_numeric() {
            continue;
        };
        if let Some(res) = find_numbers(item, grid, &mut visited, false) {
            nums.extend(res)
        }
    }

    nums.into_iter().sum()
}
//
fn solution_pt2(grid: &DefaultGrid<char>) -> i32 {
    let mut nums: Vec<i32> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, x, item) in grid.iter_enum() {
        if **item == '.' || item.is_numeric() {
            continue;
        };
        if let Some(res) = find_numbers(item, grid, &mut visited, true) {
            nums.push(res.iter().product())
        }
    }

    nums.into_iter().sum()
}

pub fn main() {
    let puzzle = get_puzzle("23", "3");
    let grid: Grid<char, ()> = Grid::new(&puzzle, Sided::Eight);

    time_it!("Part 1", solution_pt1(&grid));
    time_it!("Part 2", solution_pt2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn solution1() {
        let grid = Grid::new(TEST, Sided::Eight);
        let res = solution_pt1(&grid);
        assert_eq!(res, 4361)
    }

    #[test]
    fn solution2() {
        let grid = Grid::new(TEST, Sided::Eight);
        let res = solution_pt2(&grid);
        assert_eq!(res, 467835)
    }
}
