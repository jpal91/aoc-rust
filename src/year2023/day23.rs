#![allow(unused)]
use std::fs::read_to_string;
use std::path::PathBuf;
use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::max;

const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];




#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<char>,
    rows: i32,
    cols: i32,
    y: i32,
    x: i32
}


impl Grid {
    fn from_string(input: &str) -> Self {
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
            x: 0
        }
    }

    fn convert_coords(&self, y: i32, x: i32) -> i32 {
        y * self.rows + x
    }

    fn get_cell(&self, y:i32, x:i32) -> &char {
        let loc: i32 = self.convert_coords(y, x);
        &self.grid[loc as usize]
    }

    fn get_neighbor(&self, y: i32, x: i32, offset: i32) -> Option<(i32, i32)> {
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

    fn get_neighbors(&self, y: i32, x: i32) -> Vec<(i32, i32)> {
        let mut neighbors: Vec<(i32, i32)> = vec![];

        for d in DELTAS {
            let new_y: i32 = (y + d.0);
            let new_x: i32 = (x + d.1);
            
            if (
                new_y >= self.rows.try_into().unwrap() || 
                new_x >= self.cols.try_into().unwrap() || 
                new_y < 0 || 
                new_x < 0
            ) {
                continue
            };
            
            neighbors.push((new_y, new_x));
        };

        neighbors
    }

    fn taxi_cab(&self, n_list: &Vec<(i32, i32)>, last: &(i32, i32)) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let (ly, lx) = last;
        for (y, x) in n_list {
            res.push((ly - y).abs() + (lx - x).abs())
        }
        res
    }

    fn high_cost_neighbors(&self, y: i32, x: i32, last: (i32, i32)) {
        let neighbors = self.get_neighbors(y, x);
        let costs = self.taxi_cab(&neighbors, &last);
        let res = neighbors
            .iter()
            .zip(
                costs.iter()
            )
            .collect::<Vec<_>>()
            .sort_by(|a, b| a.1.cmp(&b.1));

        res
            
    }

    fn iter(&self) -> GridIter<'_> {
        GridIter {
            grid: &self.grid,
            rows: self.rows,
            cols: self.cols,
            x: 0,
            y: 0
        }
    }
}

struct GridIter<'a> {
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





fn bfs(grid: Grid) -> i32 {
    let mut queue: VecDeque<((i32, i32), i32, HashSet<(i32, i32)>)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut longest: i32 = 0;
    let mut last: Option<(i32, i32)> = None;

    for ((y, x), cell) in grid.iter() {
        if y == 0 && cell == '.' {
            queue.push_back(((y, x), 0, visited.clone()));
        } else if y == grid.rows - 1 && cell == '.' {
            last = Some((y, x));
            break
        } else {
            continue
        }
    };
        
    let last: (i32, i32) = last.unwrap();

    'outer: while let Some((coords, steps, mut visited)) = queue.pop_back(){

        if coords == last {
            longest = max(longest, steps);
            continue
        };

        if visited.contains(&coords) {
            continue
        }

        visited.insert(coords);
        
        // let mut current_history = history.clone();
        // current_history.entry(coords).or_insert_with(HashSet::new);
        // current_history.get_mut(&coords).unwrap().insert(coords);
        // let (y, x) = coords;

        // match grid.get_cell(coords.0, coords.1) {
        //     '>'  => {
        //         if let Some(new_coord) = grid.get_neighbor(y, x, 1) {
        //             queue.push_back((new_coord, steps + 1, visited.clone()));
        //         }
        //         continue 'outer
        //     },
        //     'v' => {
        //         if let Some(new_coord) = grid.get_neighbor(y, x, 2) {
        //             queue.push_back((new_coord, steps + 1, visited.clone()));
        //         }
        //         continue 'outer
        //     },
        //     '<' => {
        //         if let Some(new_coord) = grid.get_neighbor(y, x, 3) {
        //             queue.push_back((new_coord, steps + 1, visited.clone()));
        //         }
        //         continue 'outer
        //     },
        //     _ => {}
        // };

        let neighbors = grid.get_neighbors(coords.0, coords.1);

        for cell in neighbors {
            let y: i32 = cell.0.to_owned();
            let x: i32 = cell.1.to_owned();
            let val: char = grid.get_cell(y, x).to_owned();
            
            if visited.contains(&cell) || val == '#' {
                continue
            }

            queue.push_back(((y, x), steps + 1, visited.clone()))
        }
    };

    longest

}


pub fn main() {
    let path: PathBuf = ["input", "y23", "day23.txt"].iter().collect();
    let data = read_to_string(&path).expect("Not there");

    let grid = Grid::from_string(&data);
    let res = bfs(grid);

    println!("{res}")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn print_grid() {
        let input = "abc\ndef\nghi";
        let grid = Grid::from_string(input);

        for y in 0..grid.rows {
            let mut output = "";
            for x in 0..grid.cols {
                let item = grid.get_cell(y, x);
                println!("({:?},{:?}) {item}", y, x)
            }
        }
    }

    #[test]
    fn iter_grid() {
        let input = "abc\ndef\nghi";
        let grid = Grid::from_string(input);

        for ((y, x), cell) in grid.iter() {
            println!("({:?},{:?}) {cell}", y, x)
        }
    }
}