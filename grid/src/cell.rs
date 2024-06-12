use std::hash::Hash;
use std::ops::Deref;

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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default)]
pub struct Cell<T, E> {
    pub val: T,
    pub y: usize,
    pub x: usize,
    pub extras: E,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default, Copy)]
pub struct Cursor {
    pub coords: (i32, i32),
    pub direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default, Copy)]
pub enum Direction {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl<'cell, T, E> Cell<T, E>
where
    T: Default + Clone + 'cell,
    E: Default + Clone,
{
    pub fn new(val: T, y: usize, x: usize) -> Self {
        Self::with_extras(val, y, x, E::default())
    }

    pub fn with_extras(val: T, y: usize, x: usize, extras: E) -> Self {
        Self { val, y, x, extras }
    }

    pub fn add_extras<X>(self, extras: X) -> Cell<T, X>
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

    pub fn get_neighbor(&self, offset: usize) -> (i32, i32) {
        let (y, x) = DELTAS[offset];
        (self.y as i32 + y, self.x as i32 + x)
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

impl<T, E> Deref for Cell<T, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
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

impl IntoCell<'_, char> for char {
    fn from_str<E>(val: &'_ str, y: usize, x: usize) -> Cell<char, E>
    where
        E: Default + Clone,
    {
        Cell::new(val.chars().next().unwrap(), y, x)
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

impl_into_cell!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64);

impl Cursor {
    pub fn new(coords: (i32, i32), direction: Direction) -> Self {
        Self { coords, direction }
    }

    pub fn forward(&self) -> Self {
        let coords = match self.direction {
            Direction::North => (self.coords.0 - 1, self.coords.1),
            Direction::South => (self.coords.0 + 1, self.coords.1),
            Direction::East => (self.coords.0, self.coords.1 + 1),
            Direction::West => (self.coords.0, self.coords.1 - 1),
            _ => unimplemented!(),
        };

        Self {
            coords,
            direction: self.direction,
        }
    }

    pub fn left(&self) -> Self {
        let (y, x) = self.coords;
        let (direction, coords) = match self.direction {
            Direction::North => (Direction::West, (y, x - 1)),
            Direction::East => (Direction::North, (y - 1, x)),
            Direction::South => (Direction::East, (y, x + 1)),
            Direction::West => (Direction::South, (y + 1, x)),
            _ => unimplemented!(),
        };

        Self { direction, coords }
    }

    pub fn right(&self) -> Self {
        let (y, x) = self.coords;
        let (direction, coords) = match self.direction {
            Direction::North => (Direction::East, (y, x + 1)),
            Direction::East => (Direction::South, (y + 1, x)),
            Direction::South => (Direction::West, (y, x - 1)),
            Direction::West => (Direction::North, (y - 1, x)),
            _ => unimplemented!(),
        };

        Self { direction, coords }
    }

    pub fn reverse(&self) -> Self {
        let (y, x) = self.coords;
        let (direction, coords) = match self.direction {
            Direction::North => (Direction::South, (y + 1, x)),
            Direction::East => (Direction::West, (y, x - 1)),
            Direction::South => (Direction::North, (y - 1, x)),
            Direction::West => (Direction::East, (y, x + 1)),
            _ => unimplemented!(),
        };

        Self { direction, coords }
    }
}

impl Deref for Cursor {
    type Target = (i32, i32);

    fn deref(&self) -> &Self::Target {
        &self.coords
    }
}

pub trait Directional {
    fn left(&self) -> Direction;

    fn right(&self) -> Direction;

    fn reverse(&self) -> Direction;
}

impl<T> Directional for Cell<T, Direction> {
    fn left(&self) -> Direction {
        match self.extras {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            _ => unreachable!(),
        }
    }

    fn right(&self) -> Direction {
        match self.extras {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            _ => unreachable!(),
        }
    }

    fn reverse(&self) -> Direction {
        match self.extras {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            _ => unreachable!(),
        }
    }
}
