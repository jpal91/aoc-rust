#![allow(unused)]
use std::fmt::Debug;
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

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
    pub rows: usize,
    pub cols: usize,
    n_sides: Sided,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub struct Cell<T> {
    pub val: T,
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

    pub fn neighbors(&self, cell: &Cell<T>) -> Vec<&Cell<T>> {
        // cell
        //     .neighbors(&self.n_sides)
        //     .iter()
        let neighbors = cell.neighbors();

        let iter = match self.n_sides {
            Sided::Eight => neighbors.iter().step_by(1),
            Sided::Four => neighbors.iter().step_by(2)
        };
        
        iter
            .filter_map(|(y, x)| {
                (
                    *y >= 0 || *y < self.rows as i32 ||
                    *x >= 0 || *x < self.cols as i32
                ).then_some(&self[(*y as usize, *x as usize)])
            })
            .collect::<Vec<_>>()
    }
}

impl<T> Cell<T>
where
    T: FromStr + PartialEq + Default,
    <T as FromStr>::Err: Debug,
{
    fn new(input: &str, y: i32, x: i32) -> Self {
        let val = input.parse::<T>().unwrap();
        Cell { val, y, x }
    }

    // fn neighbors(&self, n: &Sided) -> Vec<(i32, i32)> {
    //     let iter = match n {
    //         Sided::Four => DELTAS.iter().step_by(2),
    //         Sided::Eight => DELTAS.iter().step_by(1)
    //     };

    //     iter.map(|(y, x)| (self.y + y, self.x + x)).collect()
    // }

    // pub fn coords(&self) -> (usize, usize) {
    //     (self.y as usize, self.x as usize)
    // }
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

pub trait CellLike<T> 
where
    T: FromStr + PartialEq + Default,
    <T as FromStr>::Err: Debug,
{

    fn coords(&self) -> (usize, usize);

    fn neighbors(&self) -> Vec<(i32, i32)> {
        let (row, col) = self.coords();
        DELTAS.iter().map(|(y, x)| (row as i32 + y, col as i32 + x)).collect()
    }
}

impl<T> CellLike<T> for Cell<T> 
where
    T: FromStr + PartialEq + Default,
    <T as FromStr>::Err: Debug,
{
    fn coords(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
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