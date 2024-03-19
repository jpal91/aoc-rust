use std::collections::HashSet;

use crate::utils::grid2::{Grid, Cell, CellLike};

fn escape<'a: 'b, 'b>(grid: &'a Grid<u8>, cell: &'b Cell<u8>, mut visited: &mut HashSet<&'b Cell<u8>>, height: u8) -> bool {
    if visited.contains(cell) {
        return false
    }

    visited.insert(cell);
    
    for c in grid.neighbors(cell){
        if c.val >= height {
            continue
        }
        
        let (y, x) = c.coords();

        if y == 0 || x == 0 || y == grid.rows -1 || x == grid.cols - 1 {
            return true
        }

        if escape(grid, c, visited, height) {
            return true
        }
    }

    // grid.neighbors(cell).into_iter().any(move |r| escape(grid, r, visited))
    false
}

fn solution_pt1(input: &str) -> usize {
    let grid = Grid::<u8>::new(input);
    let mut valid = (grid.rows * 2) + (grid.cols * 2) - 4;

    for cell in grid.iter() {
        let (y, x) = cell.coords();
        let mut visited: HashSet<&Cell<u8>> = HashSet::new();

        if y == 0 || x == 0 || y == grid.rows -1 || x == grid.cols - 1 {
            continue
        }

        if escape(&grid, cell, &mut visited, cell.val) {
            valid += 1
        }
        println!("{:?} {}", cell, valid);
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &'static str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_solution_1() {
        assert_eq!(solution_pt1(TEST_CASE), 21)
    }

}

