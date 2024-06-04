#![allow(unused)]

use std::fmt::Debug;
use std::ops::{Deref, Index, IndexMut};

pub type DefaultGrid<T> = Grid<Cell<T>>;

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

#[derive(Debug, PartialEq)]
pub enum Sided {
    Four,
    Eight,
}

#[derive(Debug, PartialEq)]
pub struct Grid<C> {
    grid: Vec<C>,
    pub rows: usize,
    pub cols: usize,
    n_neighbors: Sided,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Cell<T> {
    pub val: T,
    y: usize,
    x: usize,
}

pub struct GridIter<'i, C> {
    grid: &'i [C],
    idx: usize,
}

impl<'g, C> Grid<C>
where
    C: CellLike<'g> + Debug,
{
    pub fn new(input: &'g str, neighbors: Sided) -> Self {
        let mut g = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .split("")
                    .filter(|v| !v.trim().is_empty())
                    .enumerate()
                    .map(|(x, c)| C::new_from_str(c, y, x))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = g.len();
        let cols = g.first().unwrap_or(&Vec::new()).len();
        let grid = g.into_iter().flatten().collect::<Vec<_>>();

        Grid {
            grid,
            rows,
            cols,
            n_neighbors: neighbors,
        }
    }

    pub fn new_four_sided(input: &'static str) -> Self {
        Self::new(input, Sided::Four)
    }

    pub fn new_eight_sided(input: &'static str) -> Self {
        Self::new(input, Sided::Eight)
    }

    pub fn neighbors(&self, cell: &C) -> Vec<&C> {
        let neighbors = cell.neighbors();

        let iter = match self.n_neighbors {
            Sided::Four => neighbors.iter().step_by(1),
            Sided::Eight => neighbors.iter().step_by(2),
        };

        iter.filter_map(|(y, x)| {
            (*y >= 0 || *y < self.rows as i32 || *x >= 0 || *x < self.cols as i32)
                .then_some(&self[(*y as usize, *x as usize)])
        })
        .collect::<Vec<_>>()
    }

    pub fn iter(&self) -> impl Iterator<Item = &C> {
        self.grid.iter()
    }

    pub fn iter_enum(&'g self) -> impl Iterator<Item = (usize, usize, &C)> {
        GridIter {
            grid: &self.grid,
            idx: 0,
        }
    }

    pub fn transpose(self) -> Self
    where
        C: Clone,
    {
        let mut iters: Vec<_> = self
            .grid
            .chunks(self.cols)
            .map(|c| c.iter())
            .rev()
            .collect();
        let grid = (0..self.cols)
            .flat_map(|y| {
                iters
                    .iter_mut()
                    .enumerate()
                    .map(|(x, cell)| cell.next().unwrap().to_owned().with_coords(y, x))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<C>>();

        Grid {
            grid,
            rows: self.cols,
            cols: self.rows,
            n_neighbors: self.n_neighbors,
        }
    }

    pub fn add_row(&mut self, first: bool)
    where
        C: Default + Clone,
    {
        let num_rows = self.rows;
        let new_row = (0..self.cols).map(|c| {
            if first {
                C::default().with_coords(0, c)
            } else {
                C::default().with_coords(num_rows + 1, c)
            }
        });

        self.rows += 1;

        if !first {
            self.grid.extend(new_row);
            return;
        }

        let updated_grid = self.grid.clone().into_iter().map(|c| {
            let (y, x) = c.coords();
            c.with_coords(y + 1, x)
        });

        self.grid = new_row.chain(updated_grid).collect();
    }

    pub fn add_col(&mut self, first: bool)
    where
        C: Default + Clone,
    {
        let num_cols = self.cols;
        let mut new_col = (0..self.rows).map(|c| {
            if first {
                C::default().with_coords(c, 0)
            } else {
                C::default().with_coords(c, num_cols + 1)
            }
        });

        self.grid = self
            .grid
            .chunks_mut(self.cols)
            .flat_map(|win: &mut [C]| {
                let next = new_col.next().unwrap();
                let mut v: Vec<C> = vec![];
                if first {
                    v.push(next);
                    // v.extend_from_slice(win);
                    win.iter_mut().for_each(|c| c.increment_x());
                    v.extend_from_slice(win);
                } else {
                    v.extend_from_slice(win);
                    v.push(next);
                }
                v
            })
            .collect();

        self.cols += 1;
    }

    // pub fn from_iter<I>(iter: I) -> Self
    // where
    //     I: Iterator,
    //     <I as Iterator>::Item: AsRef<[C]>,
    //     C: Clone,
    // {
    //     let grid = iter.map(|row| row.as_ref().to_vec()).collect::<Vec<_>>();
    //
    //     let rows = grid.len();
    //     let cols = grid.first().unwrap_or(&Vec::new()).len();
    //
    //     Self {
    //         grid: grid.into_iter().flatten().collect(),
    //         rows,
    //         cols,
    //         n_neighbors: Sided::Four,
    //     }
    // }
}

impl<'g, C> Index<(usize, usize)> for Grid<C>
where
    C: CellLike<'g>,
{
    type Output = C;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        // let coords = (index.0 * self.rows) + index.1;
        let coords = (self.cols * index.0) + index.1;
        &self.grid[coords]
    }
}

impl<'g, C> IndexMut<(usize, usize)> for Grid<C>
where
    C: CellLike<'g>,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        // let coords = (index.0 * self.rows) + index.1;
        let coords = (self.cols * index.0) + index.1;
        &mut self.grid[coords]
    }
}

impl<'i, C> Iterator for GridIter<'i, C>
where
    C: CellLike<'i>,
{
    type Item = (usize, usize, &'i C);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.grid.len() {
            return None;
        }

        let item = &self.grid[self.idx];

        let (x, y) = item.coords();

        self.idx += 1;

        Some((y, x, item))
    }
}

impl<T> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Cell {
            val: value,
            y: 0,
            x: 0,
        }
    }
}

pub trait CellLike<'cell> {
    fn new_from_str(val: &'cell str, y: usize, x: usize) -> Self;

    fn with_coords(self, y: usize, x: usize) -> Self;

    fn coords(&self) -> (usize, usize);

    fn coords_mut(&mut self) -> (&mut usize, &mut usize);

    fn set_y(&mut self, val: usize) {
        let (y, _) = self.coords_mut();
        *y = val;
    }

    fn set_x(&mut self, val: usize) {
        let (_, x) = self.coords_mut();
        *x = val;
    }

    fn increment_x(&mut self) {
        let (_, x) = self.coords_mut();
        *x += 1;
    }

    fn increment_y(&mut self) {
        let (y, _) = self.coords_mut();
        *y += 1;
    }

    fn neighbors(&self) -> Vec<(i32, i32)> {
        let (row, col) = self.coords();
        DELTAS
            .iter()
            .map(|(y, x)| (row as i32 + y, col as i32 + x))
            .collect()
    }
}

macro_rules! impl_cell_like {
    () => {};

    ( $($type:ty $(,)?)* ) => {
        $(
            impl CellLike<'_> for Cell<$type> {
                fn new_from_str(val: &str, y: usize, x: usize) -> Self {
                    Cell {
                        val: val.trim().parse().unwrap_or_default(),
                        y,
                        x,
                    }
                }
                fn with_coords(mut self, y: usize, x: usize) -> Self {
                    // Cell {
                    //     val: self.val,
                    //     y: y as i32,
                    //     x: x as i32,
                    // }
                    self.y = y;
                    self.x = x;
                    self
                }
                fn coords(&self) -> (usize, usize) {
                    (self.y, self.x)
                }

                fn coords_mut(&mut self) -> (&mut usize, &mut usize) {
                    (&mut self.y, &mut self.x)
                }
            }
        )*
    };
}

impl_cell_like!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl CellLike<'_> for Cell<String> {
    fn new_from_str(val: &str, y: usize, x: usize) -> Self {
        Cell {
            val: val.to_owned(),
            y,
            x,
        }
    }
    fn with_coords(mut self, y: usize, x: usize) -> Self {
        self.y = y;
        self.x = x;
        self
    }
    fn coords(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn coords_mut(&mut self) -> (&mut usize, &mut usize) {
        (&mut self.y, &mut self.x)
    }
}

impl<'cell> CellLike<'cell> for Cell<&'cell str> {
    fn new_from_str(val: &'cell str, y: usize, x: usize) -> Self {
        Cell { val, y, x }
    }
    fn with_coords(mut self, y: usize, x: usize) -> Self {
        self.y = y;
        self.x = x;
        self
    }
    fn coords(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn coords_mut(&mut self) -> (&mut usize, &mut usize) {
        (&mut self.y, &mut self.x)
    }
}

#[cfg(test)]
const TEST_GRID: &str = "\
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";

#[cfg(test)]
mod tests {
    use super::*;

    fn expected<'e, T>(rows: usize, cols: usize) -> Grid<Cell<T>>
    where
        Cell<T>: CellLike<'e>,
    {
        Grid {
            grid: (0..rows)
                .flat_map(|i| (0..cols).map(move |j| Cell::new_from_str("0", i, j)))
                .collect(),
            rows,
            cols,
            n_neighbors: Sided::Four,
        }
    }

    #[test]
    fn basic_grid() {
        let grid: DefaultGrid<u32> = Grid::new(TEST_GRID, Sided::Four);
        let expected = expected::<u32>(5, 5);
        assert_eq!(grid, expected);
    }

    #[test]
    fn odd_size() {
        let grid: DefaultGrid<u32> = Grid::new(
            &TEST_GRID.lines().take(4).collect::<Vec<_>>().join("\n"),
            Sided::Four,
        );
        let expected = expected::<u32>(4, 5);
        assert_eq!(grid, expected);
    }

    #[test]
    fn diff_types() {
        let grid = DefaultGrid::<u8>::new(TEST_GRID, Sided::Four);
        let expect = expected::<u8>(5, 5);
        assert_eq!(grid, expect);

        let grid: DefaultGrid<i16> = Grid::new(TEST_GRID, Sided::Four);
        let expect = expected::<i16>(5, 5);
        assert_eq!(grid, expect);

        let grid: DefaultGrid<String> = Grid::new(TEST_GRID, Sided::Four);
        let expect = expected::<String>(5, 5);
        assert_eq!(grid, expect);

        let grid: DefaultGrid<&str> = Grid::new(TEST_GRID, Sided::Four);
        let expect = expected::<&str>(5, 5);
        assert_eq!(grid, expect);
    }

    #[test]
    fn test_iter_enum() {
        let grid = DefaultGrid::<u8>::new(TEST_GRID, Sided::Four);

        let mut iter = grid.iter_enum();

        let mut last: (usize, usize, &Cell<u8>) = iter.next().unwrap();

        assert_eq!(last, (0, 0, &Cell { val: 0, y: 0, x: 0 }));

        for i in iter {
            last = i;
        }

        assert_eq!(last, (4, 4, &Cell { val: 0, y: 4, x: 4 }));
    }

    #[test]
    fn add_row_first() {
        let mut grid = DefaultGrid::<u8>::new(TEST_GRID, Sided::Four);
        grid[(0, 0)].val = 1;
        grid.add_row(true);

        let mut expect = expected::<u8>(6, 5);
        expect[(1, 0)].val = 1;

        assert_eq!(grid, expect);
    }

    #[test]
    fn add_column() {
        let mut grid = DefaultGrid::<u8>::new(TEST_GRID, Sided::Four);
        grid[(0, 0)].val = 1;
        grid.add_col(true);

        let mut expect = expected::<u8>(5, 6);
        expect[(0, 1)].val = 1;

        assert_eq!(grid, expect);
    }

    #[test]
    fn iterator() {
        // let mut grid: DefaultGrid<u8> = Grid::from_iter([[0, 0, 0], [0, 0, 0], [0, 0, 0]].iter());
        //
        // Vec::from
        // let expect = expected::<u8>(3, 3);
        //
        // assert_eq!(grid, expect);
    }
}
