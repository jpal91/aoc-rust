#![allow(unused)]

use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::{Deref, Index, IndexMut};

pub type DefaultGrid<T> = Grid<T, ()>;

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
pub struct Grid<T, E> {
    pub grid: Vec<Cell<T, E>>,
    pub rows: usize,
    pub cols: usize,
    pub n_neighbors: Sided,
    pub default: T,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Cell<T, E> {
    pub val: T,
    pub y: usize,
    pub x: usize,
    pub extras: E,
}

pub struct GridIter<'i, C> {
    grid: &'i [C],
    idx: usize,
}

impl<'g, T, E> Grid<T, E>
where
    T: Default + Clone + 'g,
    E: Default + Clone,
{
    pub fn new(input: &'g str, neighbors: Sided) -> Self
    where
        T: IntoCell<'g, T>,
    {
        let mut g = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .split("")
                    .filter(|v| !v.trim().is_empty())
                    .enumerate()
                    .map(|(x, c)| T::from_str(c, y, x))
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
            default: T::default(),
        }
    }

    pub fn with_extras<X>(mut self, extras: X) -> Grid<T, X>
    where
        X: Default + Clone,
    {
        let grid = self
            .grid
            .into_iter()
            .map(|c| c.add_extras(extras.clone()))
            .collect();

        Grid {
            grid,
            rows: self.rows,
            cols: self.cols,
            n_neighbors: self.n_neighbors,
            default: T::default(),
        }
    }

    pub fn new_four_sided(input: &'static str) -> Self
    where
        T: IntoCell<'g, T>,
    {
        Self::new(input, Sided::Four)
    }

    pub fn new_eight_sided(input: &'static str) -> Self
    where
        T: IntoCell<'g, T>,
    {
        Self::new(input, Sided::Eight)
    }

    pub fn set_default_value(&mut self, value: T) {
        self.default = value;
    }

    pub fn get_cell(&self, y: usize, x: usize) -> Option<&Cell<T, E>> {
        (y < self.rows && x < self.cols).then(|| &self[(y, x)])
    }

    pub fn get_cell_mut(&mut self, y: usize, x: usize) -> Option<&mut Cell<T, E>> {
        (y < self.rows && x < self.cols).then(|| &mut self[(y, x)])
    }

    pub fn neighbors<C>(&self, cell: C) -> Vec<&Cell<T, E>>
    where
        C: Into<Vec<(i32, i32)>>,
    {
        let neighbors: Vec<(i32, i32)> = cell.into();

        let iter = match self.n_neighbors {
            Sided::Four => neighbors.into_iter().step_by(2),
            Sided::Eight => neighbors.into_iter().step_by(1),
        };

        iter.filter_map(|(y, x)| {
            if y >= 0 && x >= 0 {
                self.get_cell(y as usize, x as usize)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
    }

    pub fn neighbors_mut<C>(&mut self, cell: C) -> Vec<&mut Cell<T, E>>
    where
        C: Into<Vec<(i32, i32)>>,
    {
        let mut neighbors: Vec<(i32, i32)> = cell.into();

        let iter = match self.n_neighbors {
            Sided::Four => neighbors.into_iter().step_by(2),
            Sided::Eight => neighbors.into_iter().step_by(1),
        };

        let mut vec: Vec<&mut Cell<T, E>> = vec![];

        let items = iter
            .filter_map(|(y, x)| {
                if y >= 0 && x >= 0 && (y as usize) < self.rows && (x as usize) < self.cols {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        self.grid
            .iter_mut()
            .filter(|c| items.contains(&c.coords()))
            .collect::<Vec<_>>()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell<T, E>> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell<T, E>> {
        self.grid.iter_mut()
    }

    pub fn iter_enum(&'g self) -> impl Iterator<Item = (usize, usize, &Cell<T, E>)> {
        GridIter {
            grid: &self.grid,
            idx: 0,
        }
    }

    pub fn transpose(self) -> Self {
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
            .collect::<Vec<_>>();

        Grid {
            grid,
            rows: self.cols,
            cols: self.rows,
            n_neighbors: self.n_neighbors,
            default: self.default,
        }
    }

    pub fn add_row(&mut self, first: bool) {
        let num_rows = self.rows;
        let new_row = (0..self.cols).map(|c| {
            if first {
                Cell::new(self.default.clone(), 0, c)
            } else {
                Cell::new(self.default.clone(), num_rows + 1, c)
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

    pub fn add_col(&mut self, first: bool) {
        let num_cols = self.cols;
        let mut new_col = (0..self.rows).map(|c| {
            if first {
                Cell::new(self.default.clone(), c, 0)
            } else {
                Cell::new(self.default.clone(), c, num_cols + 1)
            }
        });

        self.grid = self
            .grid
            .chunks_mut(self.cols)
            .flat_map(|win: &mut [Cell<T, E>]| {
                let next = new_col.next().unwrap();
                let mut v: Vec<Cell<T, E>> = vec![];
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
}

impl<T, E> Index<(usize, usize)> for Grid<T, E> {
    type Output = Cell<T, E>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let coords = (self.cols * index.0) + index.1;
        &self.grid[coords]
    }
}

impl<T, E> IndexMut<(usize, usize)> for Grid<T, E> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let coords = (self.cols * index.0) + index.1;
        &mut self.grid[coords]
    }
}

impl<'i, T, E> Iterator for GridIter<'i, Cell<T, E>>
where
    T: Default + Clone + 'i,
    E: Default + Clone,
{
    type Item = (usize, usize, &'i Cell<T, E>);

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

impl<'cell, T, E> Cell<T, E>
where
    T: Default + Clone + 'cell,
    E: Default + Clone,
{
    pub fn new(val: T, y: usize, x: usize) -> Self {
        Self::with_extras(val, y, x, Default::default())
    }

    pub fn with_extras(val: T, y: usize, x: usize, extras: E) -> Self {
        Self { val, y, x, extras }
    }

    pub fn add_extras<X>(mut self, extras: X) -> Cell<T, X>
    where
        X: Default + Clone,
    {
        Cell {
            val: self.val,
            y: self.y,
            x: self.x,
            extras,
        }
    }

    pub fn with_coords(mut self, y: usize, x: usize) -> Self {
        self.y = y;
        self.x = x;
        self
    }

    pub fn increment_x(&mut self) {
        self.x += 1;
    }

    pub fn increment_y(&mut self) {
        self.y += 1;
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    pub fn neighbors(&self) -> Vec<(i32, i32)> {
        let (row, col) = self.coords();
        DELTAS
            .iter()
            .map(|(y, x)| (row as i32 + y, col as i32 + x))
            .collect()
    }
}

impl<I, E> AsMut<Cell<I, E>> for Cell<I, E> {
    fn as_mut(&mut self) -> &mut Cell<I, E> {
        self
    }
}

impl<T, E> From<&Cell<T, E>> for Vec<(i32, i32)>
where
    T: Default + Clone,
    E: Default + Clone,
{
    fn from(value: &Cell<T, E>) -> Self {
        value.neighbors()
    }
}

impl<I, E, A> FromIterator<A> for Grid<I, E>
where
    I: Default + Clone,
    E: Default + Clone,
    A: AsRef<[I]>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let grid = iter
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.as_ref()
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(j, col)| Cell::new(col, i, j))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = grid.len();
        let cols = grid.first().unwrap_or(&Vec::new()).len();

        Grid {
            grid: grid.into_iter().flatten().collect(),
            rows,
            cols,
            n_neighbors: Sided::Four,
            default: I::default(),
        }
    }
}

pub trait IntoCell<'cell, T> {
    fn from_str<E>(val: &'cell str, y: usize, x: usize) -> Cell<T, E>
    where
        E: Default + Clone;
}

impl<'cell> IntoCell<'cell, &'cell str> for &str {
    fn from_str<E>(val: &'cell str, y: usize, x: usize) -> Cell<&'cell str, E>
    where
        E: Default + Clone,
    {
        Cell::new(val, y, x)
    }
}

impl IntoCell<'_, String> for String {
    fn from_str<E>(val: &'_ str, y: usize, x: usize) -> Cell<String, E>
    where
        E: Default + Clone,
    {
        Cell::new(val.to_string(), y, x)
    }
}

macro_rules! impl_into_cell {
    () => {};

    ( $($type:ty $(,)?)* ) => {
        $(
            impl IntoCell<'_, $type> for $type {
                fn from_str<E>(val: &str, y: usize, x: usize) -> Cell<$type, E>
                where
                    E: Default + Clone,
                {
                    Cell::new(val.parse().unwrap(), y, x)
                }
            }
        )*
    };
}

impl_into_cell!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
