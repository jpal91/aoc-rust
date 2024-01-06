// use std::path::Iter;


// #[derive(Clone)]
// enum CellT {
//     Cell { value: String, neighbors: Vec<Box<CellT>> },
//     Nil
// }


// struct Grid {
//     grid: Vec<Vec<CellT>>,
//     y: usize,
//     x: usize,
//     size: usize
// }

// impl Grid {
//     fn new(size: usize) -> Self {
//         Grid {
//             grid: vec![vec![CellT::Cell{ value: "0".to_string(), neighbors: vec![Box::new(CellT::Nil);4]}; size]; size],
//             y: 0,
//             x: 0,
//             size: size
//         }
//     }
// }
// impl CellT {}

// impl Iterator for Grid {
//     type Item = &CellT;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.y > self.size {
//             return None;
//         } else if self.x > self.size {
//             self.x = 0;
//             self.y += 1;
//         } else {
//             self.x += 1;
//         };
        
//         Some(&self.grid[self.y][self.x])

        

//     }
// }