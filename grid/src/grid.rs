#![allow(unused)]

use std::ops::{Index, IndexMut};

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
    y: i32,
    x: i32,
}

pub struct GridIter<'i, C> {
    grid: &'i [C],
    idx: usize,
}

impl<'g, C> Grid<C>
where
    C: CellLike<'g>,
{
    pub fn new(input: &'g str, neighbors: Sided) -> Self {
        let mut g = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .split(' ')
                    .filter(|v| !v.is_empty())
                    .enumerate()
                    .map(|(x, c)| C::new_from_str(c, y as i32, x as i32))
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
                    .map(|(x, cell)| cell.next().unwrap().to_owned().new_from_coords(y, x))
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
}

impl<'g, C> Index<(usize, usize)> for Grid<C>
where
    C: CellLike<'g>,
{
    type Output = C;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let coords = (index.0 + self.rows) + index.1;
        &self.grid[coords]
    }
}

impl<'g, C> IndexMut<(usize, usize)> for Grid<C>
where
    C: CellLike<'g>,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let coords = (index.0 * self.rows) + index.1;
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

pub trait CellLike<'cell> {
    fn new_from_str(val: &'cell str, y: i32, x: i32) -> Self;

    fn new_from_coords(self, y: usize, x: usize) -> Self;

    fn coords(&self) -> (usize, usize);

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
                fn new_from_str(val: &str, y: i32, x: i32) -> Self {
                    Cell {
                        val: val.trim().parse().unwrap_or_default(),
                        y,
                        x,
                    }
                }
                fn new_from_coords(self, y: usize, x: usize) -> Self {
                    Cell {
                        val: self.val,
                        y: y as i32,
                        x: x as i32,
                    }
                }
                fn coords(&self) -> (usize, usize) {
                    (self.x as usize, self.y as usize)
                }
            }
        )*
    };
}

impl_cell_like!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl CellLike<'_> for Cell<String> {
    fn new_from_str(val: &str, y: i32, x: i32) -> Self {
        Cell {
            val: val.to_owned(),
            y,
            x,
        }
    }
    fn new_from_coords(self, y: usize, x: usize) -> Self {
        Cell {
            val: self.val,
            y: y as i32,
            x: x as i32,
        }
    }
    fn coords(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl<'cell> CellLike<'cell> for Cell<&'cell str> {
    fn new_from_str(val: &'cell str, y: i32, x: i32) -> Self {
        Cell { val, y, x }
    }
    fn new_from_coords(self, y: usize, x: usize) -> Self {
        Cell {
            val: self.val,
            y: y as i32,
            x: x as i32,
        }
    }
    fn coords(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
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
                .flat_map(|i| (0..cols).map(move |j| Cell::new_from_str("0", i as i32, j as i32)))
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
}
