#![allow(unused)]

const DELTAS: [(i32, i32); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];


#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<char>,
    rows: i32,
    cols: i32,
    y: i32,
    x: i32,
    an: bool
}


impl Grid {
    pub fn from_string(input: &str, all_neighbors: bool) -> Self {
        let mut grid: Vec<char> = vec![];
        let rows: i32 = input.lines().count().try_into().unwrap();
        let cols: i32 = input.lines().nth(0).unwrap().len().try_into().unwrap();
        
        grid
            .extend(
                input.lines()
                .map(|x: &str| 
                    x.chars().collect::<Vec<char>>()
                ).flatten()
            );
        
        Self {
            grid,
            rows,
            cols,
            y: 0,
            x: 0,
            an: all_neighbors
        }
    }

    fn convert_coords(&self, y: i32, x: i32) -> i32 {
        y * self.rows + x
    }

    pub fn get_cell(&self, y:i32, x:i32) -> &char {
        let loc: i32 = self.convert_coords(y, x);
        &self.grid[loc as usize]
    }

    pub fn get_neighbor(&self, y: i32, x: i32, offset: i32) -> Option<(i32, i32)> {
        let (dy, dx) = DELTAS[offset as usize];
        let new_y = dy + y;
        let new_x = dx + x;

        if (
            new_y >= self.rows || 
            new_x >= self.cols || 
            new_y < 0 || 
            new_x < 0
        ) {
            None
        } else {
            Some((new_y, new_x))
        }
        
    }

    pub fn get_neighbors(&self, y: i32, x: i32) -> Vec<(i32, i32)> {
        let mut neighbors: Vec<(i32, i32)> = vec![];


        for i in 0..8 {
            if !self.an && i % 2 != 0 {
                continue
            };

            if let Some(n) = self.get_neighbor(y, x, i) {
                neighbors.push(n)
            }
        };

        neighbors
    }

    pub fn iter(&self) -> std::slice::Iter<'_, char>{
        self.grid.iter()
    }

    pub fn iter_enum(&self) -> GridIter<'_> {
        GridIter {
            grid: &self.grid,
            rows: self.rows,
            cols: self.cols,
            x: 0,
            y: 0
        }
    }
}

pub struct GridIter<'a> {
    grid: &'a Vec<char>,
    rows: i32,
    cols: i32,
    x: i32,
    y: i32
}

impl<'a> GridIter<'a> {
    fn convert_coords(&self, y: i32, x: i32) -> i32 {
        y * self.rows + x
    }

    fn get_cell(&self, y:i32, x:i32) -> &char {
        let loc: i32 = self.convert_coords(y, x);
        &self.grid[loc as usize]
    }
}


impl<'a> Iterator for GridIter<'a> {
    type Item = ((i32, i32), char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.cols {
            self.x = 0;
            self.y += 1;
        } 

        if self.y >= self.rows {
            self.y = 0;
            self.x = 0;
            return None;
        };
        
        let res: Self::Item = ((self.y, self.x), self.get_cell(self.y, self.x).to_owned());
        self.x += 1;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_grid() -> Grid {
        Grid {
            grid: (0..9).map(|c| (c + '0' as u8) as char).collect(),
            rows: 3,
            cols: 3,
            y: 0,
            x: 0,
            an: false
        }
    }

    #[test]
    fn test_iter_enum() {
        let grid = create_grid();
        let mut iter = grid.iter_enum();
        assert_eq!(iter.next().unwrap(), ((0, 0), '0'));
        assert_eq!(iter.next().unwrap(), ((0, 1), '1'));
        assert_eq!(iter.next().unwrap(), ((0, 2), '2'));
        assert_eq!(iter.next().unwrap(), ((1, 0), '3'));
    }

    #[test]
    fn test_iter() {
        let grid = create_grid();
        let eq_grid: Vec<char> = (0..9).map(|c| ((c + '0' as u8) as char)).collect();

        for (idx, item) in grid.iter().enumerate() {
            assert_eq!(&eq_grid[idx], item)
        }

        let final_vec: Vec<char> = grid.iter().map(|c| c.to_owned()).collect();
        assert_eq!(final_vec, eq_grid);
    }

    #[test]
    fn test_neighbors() {
        let grid = create_grid();
        let neighbors = grid.get_neighbors(0, 0);

        assert_eq!(neighbors, vec![(0,1),(1,0)]);

        let neighbors = grid.get_neighbors(1, 1);

        assert_eq!(neighbors, vec![(0, 1), (1, 2), (2, 1), (1, 0)])

    }
}