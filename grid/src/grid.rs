#![allow(unused)]

use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::{Deref, Index, IndexMut};

use crate::cell::*;

pub type DefaultGrid<T> = Grid<T, ()>;
pub type Coords = (usize, usize);

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
                    .map(|(x, c)| T::from_str::<E>(c, y, x))
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

    fn gen_grid(value: T, rows: usize, cols: usize) -> Vec<Cell<T, E>> {
        (0..rows)
            .flat_map(|row| {
                (0..cols)
                    .map(|col| Cell::new(value.clone(), row, col))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn new_with_specs(
        rows: usize,
        cols: usize,
        default: Option<T>,
        sides: Option<Sided>,
    ) -> Self {
        let value = match default {
            Some(val) => val,
            None => T::default(),
        };

        let n_neighbors = match sides {
            Some(side) => side,
            None => Sided::Four,
        };

        Grid {
            default: value.clone(),
            grid: Self::gen_grid(value, rows, cols),
            rows,
            cols,
            n_neighbors,
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

    pub fn new_four_sided(input: &'g str) -> Self
    where
        T: IntoCell<'g, T>,
    {
        Self::new(input, Sided::Four)
    }

    pub fn new_eight_sided(input: &'g str) -> Self
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

    pub fn get_cell_signed(&self, coords: (i32, i32)) -> Option<&Cell<T, E>> {
        let (y, x) = coords;
        if (y >= 0 && x >= 0 && y < (self.rows as i32) && x < (self.cols as i32)) {
            Some(&self[(y as usize, x as usize)])
        } else {
            None
        }
    }

    pub fn get_neighbor(&self, coords: (usize, usize), offset: usize) -> Option<&Cell<T, E>> {
        if let Some(cell) = self.get_cell(coords.0, coords.1) {
            let (y, x) = cell.get_neighbor(offset);
            match (y >= 0, x >= 0) {
                (true, true) => self.get_cell((y as usize), (x as usize)),
                _ => None,
            }
        } else {
            None
        }
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

    pub fn all_neighbors<C>(&self, cell: C) -> Vec<Option<&Cell<T, E>>>
    where
        C: Into<Vec<(i32, i32)>>,
    {
        let neighbors: Vec<(i32, i32)> = cell.into();

        let iter = match self.n_neighbors {
            Sided::Four => neighbors.into_iter().step_by(2),
            Sided::Eight => neighbors.into_iter().step_by(1),
        };

        iter.map(|(y, x)| {
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

    pub fn coords_vec(&self) -> Vec<(usize, usize)> {
        self.grid.iter().map(|c| c.coords()).collect()
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
