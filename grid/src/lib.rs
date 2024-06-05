pub mod cell;
pub mod grid;
pub mod macros;
pub mod tests;

pub mod prelude {

    pub use super::{
        cell::{Cell, IntoCell},
        grid,
        grid::{DefaultGrid, Grid, GridIter, Sided},
    };
}
