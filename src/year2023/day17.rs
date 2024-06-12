use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Deref;
use std::usize;

use ::grid::prelude::{Coords, Cursor, Dijkstra, Direction, Directional, IntoCell};

use crate::{get_puzzle, time_it};

type Grid = ::grid::prelude::Grid<usize, ()>;
type Cell = ::grid::prelude::Cell<usize, ()>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct QueueItem {
    cell: Facing,
    cost: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Facing {
    cell: Cursor,
    count: usize,
}

impl Facing {
    fn new(coords: Coords, direction: Direction) -> Self {
        Self {
            cell: Cursor::new((coords.0 as i32, coords.1 as i32), direction),
            count: 0,
        }
    }

    fn forward(&self) -> Self {
        Self {
            cell: self.cell.forward(),
            count: self.count + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            cell: self.cell.left(),
            count: 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            cell: self.cell.right(),
            count: 1,
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        if self.count >= 3 {
            vec![self.left(), self.right()]
        } else {
            vec![self.forward(), self.left(), self.right()]
        }
    }
}

impl Deref for Facing {
    type Target = (i32, i32);

    fn deref(&self) -> &Self::Target {
        self.cell.deref()
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cell.cmp(&other.cell))
    }
}

fn _bfs(grid: &Grid, last: Coords) -> usize {
    let mut queue = BinaryHeap::new();
    let mut best_heat = usize::MAX;
    let mut heat_map = HashMap::new();
    let last = (last.0 as i32, last.1 as i32);

    for v in [
        Facing::new((0, 0), Direction::East),
        Facing::new((0, 0), Direction::South),
    ] {
        queue.push(QueueItem { cell: v, cost: 0 });
        heat_map.insert(v, 0);
    }

    while let Some(QueueItem { cell, cost }) = queue.pop() {
        if *cell == last {
            return cost;
        }

        let dist = *heat_map.get(&cell).unwrap_or(&usize::MAX);
        if cost > dist {
            continue;
        }

        for neighbor in cell.neighbors() {
            let new_cost = match grid.get_cell_signed(*neighbor) {
                Some(n) => cost + **n,
                _ => continue,
            };

            let dist_to_next = heat_map.get(&neighbor).unwrap_or(&usize::MAX);

            if new_cost < *dist_to_next {
                heat_map
                    .entry(neighbor)
                    .and_modify(|c| *c = new_cost)
                    .or_insert(new_cost);
                let next = QueueItem {
                    cell: neighbor,
                    cost: new_cost,
                };

                queue.push(next);
            }
        }
    }

    best_heat
}

fn _solution_pt1(input: &str) -> usize {
    let grid = Grid::new_four_sided(input);
    let last = ((grid.rows as i32) - 1, (grid.cols as i32) - 1);

    let end = |node: &Facing| **node == last;
    let cost = |node: &Facing| grid.get_cell_signed(**node).map(|c| c.val);
    let neighbors = |node: Facing| node.neighbors();

    let bfs = Dijkstra::new(&neighbors, &cost, &end);

    let res = bfs.cost(vec![
        Facing::new((0, 0), Direction::East),
        Facing::new((0, 0), Direction::South),
    ]);

    res.unwrap_or(0)
}

fn _solution_pt2(input: &str) -> usize {
    let grid = Grid::new_four_sided(input);
    let last = ((grid.rows as i32) - 1, (grid.cols as i32) - 1);

    let end = |node: &Facing| **node == last && node.count >= 4;
    let cost = |node: &Facing| grid.get_cell_signed(**node).map(|c| c.val);
    let neighbors = |node: Facing| {
        if node.count < 4 {
            vec![node.forward()]
        } else if node.count >= 4 && node.count < 10 {
            vec![node.forward(), node.left(), node.right()]
        } else {
            vec![node.left(), node.right()]
        }
    };

    let bfs = Dijkstra::new(&neighbors, &cost, &end);

    let res = bfs.cost(vec![
        Facing::new((0, 0), Direction::East),
        Facing::new((0, 0), Direction::South),
    ]);

    res.unwrap_or(0)
}

pub fn main() {
    let puzzle = get_puzzle("23", "17");

    time_it!("Solution Pt 1", _solution_pt1(&puzzle));
    time_it!("Solution Pt 2", _solution_pt2(&puzzle));
}

// #[cfg(test)]
const TEST_ONE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let res = _solution_pt1(TEST_ONE);

        assert_eq!(res, 102);
    }

    #[test]
    fn test_one_1() {
        let res = _solution_pt1(TEST_ONE);

        assert_eq!(res, 102);
    }
}
