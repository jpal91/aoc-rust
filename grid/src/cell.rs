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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Cell<T, E> {
    pub val: T,
    pub y: usize,
    pub x: usize,
    pub extras: E,
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
