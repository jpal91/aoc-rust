#![allow(unused)]
use std::collections::HashSet;

use crate::utils::grid::Grid;
use crate::{get_puzzle,time_it};

fn check_adjacent(y: i32, x: i32, grid: &Grid, visited: &mut HashSet<(i32, i32)>) -> i32 {
    let mut number: Vec<char> = vec![grid.get_cell(y, x).to_owned()];
    let mut ny = y;
    let mut nx = x;

    while let Some(c) = grid.get_neighbor(ny, nx, 6) {
        let next_char = grid.get_cell(c.0, c.1);

        if next_char.is_numeric() {
            number.insert(0, next_char.to_owned());
            visited.insert((c.0, c.1));
            nx -= 1;
        } else {
            break
        }
    };
    
    nx = x;


    while let Some(c) = grid.get_neighbor(ny, nx, 2) {
        let next_char = grid.get_cell(c.0, c.1);
        if next_char.is_numeric() {
            number.push(next_char.to_owned());
            visited.insert((c.0, c.1));
            nx += 1;
        } else {
            break
        }
    };

    number.iter().collect::<String>().parse::<i32>().unwrap()
}

fn find_numbers(y: i32, x: i32, grid: &Grid, visited: &mut HashSet<(i32, i32)>, part2: bool) -> Option<Vec<i32>> {

    let mut res: Vec<i32> = vec![];
    
    for (dy, dx) in grid.get_neighbors(y, x) {
        if visited.contains(&(dy, dx)) {
            continue
        };
        
        if grid.get_cell(dy, dx).is_numeric() {
            visited.insert((dy, dx));
            let adj: i32 = check_adjacent(dy, dx, grid, visited);
            res.push(adj)
        }
    };
    // res
    match (part2, res.len()) {
        (true, 2) => Some(res),
        (false, _) => Some(res),
        _ => None
    }
}

fn solution_pt1(grid: &Grid) -> i32 {
    let mut nums: Vec<i32> = vec![];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
    for ((y, x), item) in grid.iter_enum() {
        if item == '.' || item.is_numeric() {
            continue
        };
        if let Some(res) = find_numbers(y, x, grid, &mut visited, false) {
            nums.extend(res)
        }
        
    }

    nums.into_iter().sum()
}

fn solution_pt2(grid: &Grid) -> i32 {
    let mut nums: Vec<i32> = vec![];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    
    for ((y, x), item) in grid.iter_enum() {
        if item == '.' || item.is_numeric() {
            continue
        };
        if let Some(res) = find_numbers(y, x, grid, &mut visited, true) {
            nums.push(res.iter().product())
        }
    }

    nums.into_iter().sum()
}

pub fn main() {
    let puzzle = get_puzzle("23", "3");
    let grid = Grid::from_string(&puzzle, true);

    time_it!("Part 1", solution_pt1(&grid));
    time_it!("Part 2", solution_pt2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &'static str = "\
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
        let grid = Grid::from_string(TEST, true);
        let res = solution_pt1(&grid);
        assert_eq!(res, 4361)
    }

    #[test]
    fn solution2() {
        let grid = Grid::from_string(TEST, true);
        let res = solution_pt2(&grid);
        assert_eq!(res, 467835)
    }
}