#![allow(unused)]
use std::fmt::Debug;
use std::str::FromStr;
use std::ops::{Index, IndexMut};

const DELTAS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[derive(PartialEq, Debug)]
enum Sided {
    Four,
    Eight,
}

#[derive(PartialEq, Debug)]
pub struct Grid<T> {
    grid: Vec<Cell<T>>,
    rows: usize,
    cols: usize,
    n_sides: Sided,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell<T> {
    val: T,
    y: i32,
    x: i32,
}

pub struct GridIter<'a, T> {
    grid: &'a Vec<Cell<T>>,
    y: usize,
    x: usize,
    idx: usize,
}

impl<T> Grid<T>
where
    T: FromStr + PartialEq + Clone + Default + Debug,
    <T as FromStr>::Err: Debug,
{
    pub fn new(input: &str) -> Self {
        let mut g = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l
                    .trim()
                    .split("")
                    .filter(|v| !v.is_empty())
                    .enumerate()
                    .map(|(x, c)| Cell::<T>::new(c, y as i32, x as i32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = g.len();
        let cols = g.first().unwrap_or(&Vec::new()).len();
        let grid = g.into_iter().flatten().collect::<Vec<_>>();

        Grid { grid, rows, cols, n_sides: Sided::Four }
    }

    pub fn to_eight_sides(self) -> Self {
        Grid {
            grid: self.grid,
            rows: self.rows,
            cols: self.cols,
            n_sides: Sided::Eight
        }
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            grid: &self.grid,
            idx: 0,
            y: 0,
            x: 0
        }
    }

    pub fn transpose(self) -> Self {
        let mut iters: Vec<_> = self.grid.chunks(self.cols).map(|c| c.into_iter()).rev().collect();
        println!("{:?} {} {}", iters, self.rows, self.cols);
        let grid = (0..self.cols)
            .flat_map(|y| {
                iters
                    .iter_mut()
                    .enumerate()
                    .map(|(x, n)| Cell{ val: n.next().unwrap().val.to_owned(), y: y as i32, x: x as i32 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Cell<T>>>();

        Grid { grid, rows: self.cols, cols: self.rows, n_sides: self.n_sides }
    }
}

impl<T> Cell<T>
where
    T: FromStr + PartialEq + Default,
    <T as FromStr>::Err: Debug,
{
    fn new(input: &str, y: i32, x: i32) -> Self {
        // eprintln!("{input:?}, {}", input.is_empty());
        let val = input.parse::<T>().unwrap();
        Cell { val, y, x }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = Cell<T>;
    
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let coords = (index.0 * self.rows) + index.1;
        &self.grid[coords]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let coords = (index.0 * self.rows) + index.1;
        &mut self.grid[coords]
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a Cell<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.grid.len() {
            return None
        }

        let item = &self.grid[self.idx];
        self.idx += 1;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected<S: From<u8>>() -> Vec<Cell<S>> {
        vec![
            Cell { val: S::from(1), y: 0, x: 0 },
            Cell { val: S::from(2), y: 0, x: 1 },
            Cell { val: S::from(3), y: 0, x: 2 },
            Cell { val: S::from(4), y: 1, x: 0 },
            Cell { val: S::from(5), y: 1, x: 1 },
            Cell { val: S::from(6), y: 1, x: 2 },
            Cell { val: S::from(7), y: 2, x: 0 },
            Cell { val: S::from(8), y: 2, x: 1 },
            Cell { val: S::from(9), y: 2, x: 2 }
        ]
    }

    #[test]
    fn test_basic_grid() {
        let grid_str = "123\n456\n789";
        let expected = Grid::<u32> {
            grid: expected::<u32>(),
            rows: 3,
            cols: 3,
            n_sides: Sided::Four
        };

        assert_eq!(expected, Grid::<u32>::new(grid_str))
    }

    #[test]
    fn test_other_type() {
        let grid_str = "123\n456\n789";
        let expected = Grid::<i16> {
            grid: expected::<i16>(),
            rows: 3,
            cols: 3,
            n_sides: Sided::Four
        };

        assert_eq!(expected, Grid::<i16>::new(grid_str))
    }


    #[test]
    fn test_indexing() {
        let mut grid = Grid::<u32>::new("123\n456\n789");
        assert_eq!(&grid[(0, 2)], &Cell { val: 3, y: 0, x: 2});
        assert_eq!(&grid[(2, 1)], &Cell { val: 8, y: 2, x: 1});
        assert_eq!(&mut grid[(1, 2)], &mut Cell { val: 6, y: 1, x: 2})
    }

    #[test]
    fn test_transpose() {
        let grid1 = Grid::<u32>::new("6420\n7531");
        let grid2 = Grid::<u32>::new("01\n23\n45\n67").transpose();

        assert_eq!(grid1, grid2);

        let grid3 = Grid::<u32>::new("123\n456\n789");
        let grid4 = Grid::<u32>::new("369\n258\n147").transpose();

        assert_eq!(grid3, grid4);
    }
}