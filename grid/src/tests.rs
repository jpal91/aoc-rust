#![cfg(test)]
use crate::grid::*;

const TEST_GRID: &str = "\
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
";

fn expected<'e, T>(rows: usize, cols: usize) -> Grid<T, ()>
where
    T: IntoCell<'e, T> + Default,
{
    Grid {
        grid: (0..rows)
            .flat_map(|i| (0..cols).map(move |j| T::from_str("0", i, j)))
            .collect(),
        rows,
        cols,
        n_neighbors: Sided::Four,
        default: T::default(),
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

    let mut last: (usize, usize, &Cell<u8, ()>) = iter.next().unwrap();

    assert_eq!(
        last,
        (
            0,
            0,
            &Cell {
                val: 0,
                y: 0,
                x: 0,
                extras: ()
            }
        )
    );

    for i in iter {
        last = i;
    }

    assert_eq!(
        last,
        (
            4,
            4,
            &Cell {
                val: 0,
                y: 4,
                x: 4,
                extras: ()
            }
        )
    );
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
    let grid: DefaultGrid<u8> = Grid::from_iter([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);

    let expect = expected::<u8>(3, 3);

    assert_eq!(grid, expect);
}

#[derive(Default, Clone, PartialEq, Debug)]
struct TestExtras;

#[test]
fn extras() {
    let grid: Grid<u8, ()> = Grid::new(TEST_GRID, Sided::Four);
    let cell = &grid[(0, 0)];

    assert_eq!(
        cell,
        &Cell {
            val: 0,
            y: 0,
            x: 0,
            extras: ()
        }
    );

    let grid = grid.with_extras(TestExtras);
    let cell = &grid[(0, 0)];

    assert_eq!(cell.extras, TestExtras);
}

#[test]
fn default_value() {
    let mut grid: DefaultGrid<u8> = Grid::new(TEST_GRID, Sided::Four);
    grid.set_default_value(1);
    grid.add_row(false);

    assert_eq!(grid[(4, 0)].val, 0);
    assert_eq!(grid[(5, 0)].val, 1);
}

#[test]
fn get_cell() {
    let mut grid: Grid<u8, ()> = Grid::new(TEST_GRID, Sided::Four);
    let cell = grid.get_cell(3, 2);

    assert_eq!(
        cell,
        Some(&Cell {
            val: 0,
            y: 3,
            x: 2,
            extras: ()
        })
    );

    let cell = grid.get_cell(10, 10);

    assert_eq!(cell, None);

    let cell = grid.get_cell_mut(1, 1).unwrap();
    cell.val = 10;

    assert_eq!(grid[(1, 1)].val, 10);
}

#[test]
fn neighbors() {
    let mut grid: Grid<u8, ()> = Grid::new(TEST_GRID, Sided::Four);
    grid[(1, 2)].val = 1;
    grid[(2, 3)].val = 2;
    grid[(3, 2)].val = 3;
    grid[(2, 1)].val = 4;

    let neighbors = grid.neighbors(&grid[(2, 2)]);
    assert_eq!(neighbors.len(), 4);

    assert_eq!(
        neighbors.iter().map(|c| c.val).collect::<Vec<_>>(),
        (1..=4).collect::<Vec<_>>()
    );
}

#[test]
fn neighbors_mut() {
    let mut grid: Grid<u8, ()> = Grid::new(TEST_GRID, Sided::Four);
    let cell = &grid[(2, 2)];

    let neighbors = grid.neighbors_mut(cell.neighbors());
    let coords = neighbors.iter().map(|c| c.coords()).collect::<Vec<_>>();

    for (i, n) in neighbors.into_iter().enumerate() {
        n.val = i as u8;
    }

    for (i, (y, x)) in coords.into_iter().enumerate() {
        assert_eq!(grid[(y, x)].val, i as u8);
    }
}
// }
