use std::{
    collections::{HashSet, VecDeque},
    ops::Deref,
};

use crate::{get_puzzle, time_it};

use ::grid::prelude::{Cell, IntoCell};
use grid::grid::Sided;

type Grid = ::grid::prelude::Grid<Pipe, Looped>;
type Coords = (usize, usize);
type Queue<'q> = VecDeque<(Coords, u32)>;

#[derive(Debug, Default, Clone, PartialEq)]
enum Pipe {
    Start,
    NorthWest,
    NorthEast,
    WestSouth,
    EastSouth,
    Horizon,
    Vert,
    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, Copy)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Debug, Default, Clone)]
enum Looped {
    In,
    #[default]
    Out,
    Loop,
}

impl<'cell> IntoCell<'cell, Pipe> for Pipe {
    fn from_str<E>(val: &'cell str, y: usize, x: usize) -> Cell<Pipe, E>
    where
        E: Default + Clone,
    {
        use Pipe::*;

        let val = match val {
            "S" => Start,
            "-" => Horizon,
            "7" => WestSouth,
            "|" => Vert,
            "J" => NorthWest,
            "L" => NorthEast,
            "F" => EastSouth,
            _ => Empty,
        };

        Cell::new(val, y, x)
    }
}

trait Escape {
    fn can_escape(&self, cell: &Cell<Pipe, Looped>) -> bool;
}

impl Escape for Grid {
    fn can_escape(&self, cell: &Cell<Pipe, Looped>) -> bool {
        let (y, x) = cell.coords();

        [
            y.checked_sub(1).is_none(),
            y + 1 >= self.rows,
            x.checked_sub(1).is_none(),
            x + 1 >= self.cols,
        ]
        .iter()
        .any(|&x| x)
    }
}

impl Direction {
    fn from_target(target: Coords, dest: Coords) -> Self {
        use Direction::*;

        match (target, dest) {
            ((y1, _), (y2, _)) if y1 > y2 => North,
            ((y1, _), (y2, _)) if y1 < y2 => South,
            ((_, x1), (_, x2)) if x1 > x2 => West,
            ((_, x1), (_, x2)) if x1 < x2 => East,
            _ => unreachable!(),
        }
    }
}

fn can_enter(start: &Direction, dest: &Pipe) -> bool {
    use Direction::*;
    use Pipe::*;

    matches!(
        (start, dest),
        (North, Vert | EastSouth | WestSouth)
            | (East, Horizon | NorthWest | WestSouth)
            | (South, Vert | NorthWest | NorthEast)
            | (West, Horizon | NorthEast | EastSouth)
    )
}

fn new_direction(start: &Direction, dest: &Pipe) -> Direction {
    use Direction::*;
    use Pipe::*;

    match (start, dest) {
        (North, EastSouth) => South,
        (South, EastSouth) => East,
        (West, Horizon) => East,
        (East, Horizon) => West,
        (West, WestSouth) => South,
        (South, WestSouth) => West,
        (North, Vert) => South,
        (South, Vert) => North,
        (North, NorthWest) => West,
        (West, NorthWest) => North,
        (North, NorthEast) => East,
        (East, NorthEast) => North,
        _ => unreachable!(),
    }
}

fn bfs(grid: &mut Grid) -> u32 {
    let mut queue: Queue = VecDeque::new();
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut count = 0;

    for cell in grid.iter() {
        if **cell == Pipe::Start {
            let coords = cell.coords();
            queue.push_back((coords, 0));
            visited.insert(coords);
            break;
        }
    }

    while let Some(((y, x), steps)) = queue.pop_front() {
        count = count.max(steps);

        let cell = grid.get_cell_mut(y, x).unwrap();
        cell.extras = Looped::Loop;

        let neighbor_vec = cell.neighbors();

        for neighbor in grid.neighbors(neighbor_vec) {
            let coords = neighbor.coords();
            let direction = Direction::from_target((y, x), coords);

            if visited.contains(&coords) || !can_enter(&direction, neighbor) {
                continue;
            }

            visited.insert(coords);
            queue.push_back((coords, steps + 1));
        }
    }

    count
}

fn try_escape(grid: &mut Grid, cell: Coords) -> bool {
    let mut queue: Queue = VecDeque::from_iter([(cell, 0)]);
    let mut visited: HashSet<Coords> = HashSet::from_iter([cell]);

    while let Some(((y, x), _)) = queue.pop_front() {
        let cell = grid.get_cell(y, x).unwrap();

        if grid.can_escape(cell) {
            return true;
        }

        for neighbor in grid.neighbors(cell) {
            let coords = neighbor.coords();

            match neighbor.extras {
                _ if visited.contains(&coords) => continue,
                Looped::In | Looped::Loop => continue,
                _ => {}
            };

            visited.insert(coords);
            queue.push_back((coords, 0));
        }
    }

    false
}

fn solution_pt1(input: &str) -> u32 {
    let mut grid = Grid::new_four_sided(input);

    bfs(&mut grid)
}

fn solution_pt2(input: &str) -> u32 {
    use Looped::*;
    let mut grid = Grid::new_four_sided(input);

    let _ = bfs(&mut grid);
    let mut count = 0;

    for (y, x) in grid.coords_vec() {
        let extras = grid[(y, x)].extras.clone();

        if let Out = extras {
            println!("{} {}", y, x);
            if !try_escape(&mut grid, (y, x)) {
                grid[(y, x)].extras = In;
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let puzzle = get_puzzle("23", "10");

    time_it!("Solution Pt 1", solution_pt1(&puzzle));
}

#[cfg(test)]
const TEST_ONE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

#[cfg(test)]
const TEST_TWO: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[cfg(test)]
const TEST_THREE: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[cfg(test)]
const TEST_FOUR: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_pt1_one() {
        let res = solution_pt1(TEST_ONE);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_solutions_pt1_two() {
        let res = solution_pt1(TEST_TWO);

        assert_eq!(res, 8);
    }

    #[test]
    fn test_solution_pt2() {
        let res = solution_pt2(TEST_FOUR);

        assert_eq!(res, 4);
    }
}
