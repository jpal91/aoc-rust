use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use grid::prelude::{Cursor, Direction, IntoCell, QueueItem};

use crate::{get_puzzle, time_it};

type Grid = ::grid::prelude::Grid<Path, ()>;
type Cell = ::grid::prelude::Cell<Path, ()>;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
enum Path {
    #[default]
    Plot,
    Rock,
    Start,
}

impl IntoCell<'_, Path> for Path {
    fn from_str<E>(val: &'_ str, y: usize, x: usize) -> grid::prelude::Cell<Path, E>
    where
        E: Default + Clone,
    {
        let val = match val {
            "." => Path::Plot,
            "S" => Path::Start,
            "#" => Path::Rock,
            _ => unreachable!(),
        };

        ::grid::prelude::Cell::new(val, y, x)
    }
}

fn solution_pt1(input: &str, max: usize) -> usize {
    let grid = Grid::new_four_sided(input);
    let start = grid.find(Path::Start).unwrap();
    let mut visited: HashSet<&Cell> = HashSet::new();
    visited.insert(start);

    for _ in 0..max {
        let mut reached = HashSet::new();
        for v in visited {
            for neighbor in grid.neighbors(v) {
                if matches!(neighbor.val, Path::Plot | Path::Start) {
                    reached.insert(neighbor);
                }
            }
        }
        visited = reached;
    }

    visited.len()
}

pub fn main() {
    let puzzle = get_puzzle("23", "21");

    time_it!("Solution Pt 1", solution_pt1(&puzzle, 64));
}

#[cfg(test)]
const TEST_ONE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = solution_pt1(TEST_ONE, 6);

        assert_eq!(res, 16);
    }
}
