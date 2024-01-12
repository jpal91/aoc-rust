#![allow(unused)]
use std::collections::VecDeque;
use std::ops::Range;
use regex::Regex;

use crate::{get_puzzle, time_it};

#[derive(Debug)]
struct MapRange(u64, u64, u64);

#[derive(Debug, Clone, Copy)]
struct SliceRange(u64, u64);

#[derive(Debug)]
struct Map {
    rlist: Vec<MapRange>,
    min_val: u64,
    max_val: u64
}

#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
    min_val: u64,
    max_val: u64
}

impl SliceRange {
    fn is_valid(&self) -> bool {
        self.0 <= self.1
    } 
}

impl MapRange {
    fn get(&self, val: u64) -> Option<u64> {
        if val >= self.1 && val < (self.1 + self.2) {
            Some((val - self.1) + self.0)
        } else {
            None
        }
    }

    fn get_from_slice(&self, range: &mut SliceRange) -> Option<Vec<SliceRange>> {
        if 
            range.0 > self.1 + (self.2 - 1) ||
            range.1 < self.1 ||
            !range.is_valid()
        {
            return None;
        }
        
        let mut res: Vec<SliceRange> = vec![];
        let mut new_min: u64;
        let new_max = ((self.1 + (self.2 - 1)).min(range.1)) - self.1;

        if range.0 < self.1 {
            // res.push(SliceRange(range.0, self.1 - 1));
            new_min = 0;
            range.1 = self.1 - 1;
        } else {
            new_min = (self.1.max(range.0)) - self.1;
            range.0 = range.1 + 1;
        }


        res.push(SliceRange(new_min + self.0, new_max + self.0));

        // if range.0 < self.1 {
        //     range.1 = self.1 - 1;
        // }

        Some(res)
    }

    fn get_from_slice2(&self, range: SliceRange) -> (Option<Vec<SliceRange>>, Option<Vec<SliceRange>>) {
        let mut res: Vec<SliceRange> = vec![];
        let mut todo: Vec<SliceRange> = vec![];

        if 
            range.0 > self.1 + (self.2 - 1) ||
            range.1 < self.1 

        {
            todo.push(range);
            (None, Some(todo))
        } else if 
            range.0 < self.1 &&
            range.1 > self.1 + (self.2 - 1)
        {
            todo.extend([
                SliceRange(range.0, self.1 - 1),
                SliceRange(self.1 + (self.2 - 1), range.1)
            ]);
            res.push(SliceRange(self.0, self.0 + self.2 - 1));
            (Some(res), Some(todo))
        } else if 
            range.0 < self.1
        {
            todo.push(SliceRange(range.0, self.1 - 1));
            res.push(SliceRange(self.0, self.0 + (range.1 - self.1)));
            (Some(res), Some(todo))
        } else if 
            range.1 > self.1 + (self.2 - 1)
        {
            todo.push(SliceRange(self.1 + (self.2 - 1), range.1));
            res.push(SliceRange(range.1, self.0 + (self.2 - 1)));
            (Some(res), Some(todo))
        } else {
            res.push(SliceRange(self.0 + (range.0 - self.1), self.0 + (range.1 - self.1)));
            (Some(res), None)
        }

        
    }
}

impl Map {
    fn from_string(input: &str) -> Self {
        let mut rlist = vec![];
        let mut min_val = u64::MAX - 1;
        let mut max_val = 0;
        
        for line in input.lines() {
            // println!("{line}");
            let nums: Vec<u64> = line
                .split(" ")
                .map(|num| num.trim().parse::<u64>().unwrap())
                .collect();
            
            let map_range = MapRange(nums[0], nums[1], nums[2]);
            // println!("{} {} {}", line, min_val, max_val);
            min_val = min_val.min(map_range.1);
            max_val = max_val.max((map_range.1 + map_range.2));
            
            rlist.push(map_range);
        };

        Self {
            rlist,
            min_val,
            max_val
        }
    }

    fn get(&self, val: u64) -> u64 {
        if val >= self.max_val || val < self.min_val { return val; }
        
        for r in self.rlist.iter() {
            if let Some(res) = r.get(val) {
                return res;
            }
        }
        val
    }

    fn get_from_slice(&self, range: &mut SliceRange) -> Vec<SliceRange> {
        let mut res: Vec<SliceRange> = vec![];

        for r in self.rlist.iter() {
            if let Some(slices) = r.get_from_slice(range) {
                res.extend(slices);
            }
        }
        
        if res.is_empty() || range.is_valid() {
            res.push(*range)
        }
        res
    }

    fn get_from_slice2(&self, range: &mut SliceRange) -> Vec<SliceRange> {
        let mut res_vec: Vec<SliceRange> = vec![];
        let mut todo_vec: Vec<SliceRange> = vec![*range];

        for r in self.rlist.iter() {
            for _ in 0..todo_vec.len() {
                let (responses, todos) = r.get_from_slice2(todo_vec.pop().unwrap());

                if let Some(todo) = todos {
                    todo_vec.extend(todo);
                }

                if let Some(response) = responses {
                    res_vec.extend(response);
                }
            }
        }

        res_vec.append(&mut todo_vec);
        res_vec
    }
}

impl Almanac {
    fn new() -> Self {
        Self {
            maps: vec![],
            min_val: u64::MAX - 1,
            max_val: 0,
        }
    }

    fn add(&mut self, map: Map) {
        self.min_val = self.min_val.min(map.min_val);
        self.max_val = self.max_val.max(map.max_val);
        self.maps.push(map);
    }

    fn get(&self, val: u64) -> u64 {
        if val >= self.max_val || val < self.min_val { return val; }
        
        let mut cur_val: u64 = val;

        for map in self.maps.iter() {
            cur_val = map.get(cur_val);
        }
        cur_val
    }

    fn clamp(&self, mut range: Range<u64>) {
        let mut new_range: Option<Range<u64>> = None;
        let mut lower_bound: Option<u64> = None;
        let mut upper_bound: Option<u64> = None;

        
    }

    fn run_range(&self, range: Range<u64>) -> u64 {
        range
            .map(|i| self.get(i))
            .reduce(|acc, e| acc.min(e))
            .unwrap()
    }

    fn test(&self) {
        let mut cur_val: u64 = 0;
        for i in 0..(4294967296 as u64) {
            cur_val = self.get(i)
        }
        println!("{cur_val}")
    }

    // fn get_from_range(&self, range: Range<u64>) -> u64 {
    //     if 
    //         (range.end <= self.min_val) ||
    //         (range.start >= self.max_val)
    //     {
    //         range.min().unwrap()
    //     } else if range.start < self.min_val && range.end > self.min_val {
    //         let min_val = range.start;
    //         let new_range = (self.min_val..self.max_val.min(range.end));
    //         self.run_range(new_range).min(min_val)
    //     } else if range.start > self.min_val && range.end > self.max_val {
    //         let min_val = self.max_val + 1;
    //         let new_range = (range.start..self.max_val);
    //         self.run_range(new_range).min(min_val)
    //     } else {
    //         self.run_range(range)
    //     }
    // }

    // fn get_from_range(&self, range: Range<u64>) -> u64 {
    //     let mut min_val = u64::MAX;
        
    //     if range.start < self.min_val {
    //         min_val = range.start;
    //     };

    //     let new_range = (self.min_val.max(range.start)..self.max_val.min(range.end));
    //     self.run_range(new_range).min(min_val)
        
    // }

    fn get_from_range(&self, range: SliceRange) -> u64 {
        // let mut queue: VecDeque<(u64, Vec<SliceRange>)> = VecDeque::new();
        // queue.push_back((0, vec![range]));
        let mut min_val = u64::MAX -1;

        // while let Some((map, ranges)) = queue.pop_front() {
        //     if map == 6 {
        //         min_val = ranges
        //             .iter()
        //             .map(|s| s.0)
        //             .reduce(|acc, e| acc.min(e))
        //             .unwrap()
        //             .min(min_val);
        //         continue
        //     }  

        //     for r in ranges {
        //         let ns: Vec<SliceRange> = self.maps[map as usize]
        //             .get_from_slice(&r);
        //         queue.push_back((map + 1, ns))
        //     }
        // }
        let mut ranges = vec![range];

        for (i, map) in self.maps.iter().enumerate() {
            let mut tmp: Vec<SliceRange> = vec![];
            for r in ranges.iter_mut() {
                tmp.extend(map.get_from_slice2(r));
 
            };
            // println!("{:?}", tmp);
            ranges = tmp;
        }
        // println!("{:?} {}", ranges, min_val);
        println!("{}", min_val);
        min_val = ranges
            .iter()
            .map(|s| s.0)
            .fold(min_val, |acc, e| acc.min(e));

        min_val
    }
}

fn parse(input: &str) -> (Vec<u64>, Almanac) {
    let mut lines = input.split("\n\n");
    let seeds = lines.next().unwrap();
    

    let seeds_re = Regex::new(r"seeds: (.+)").unwrap();
    let maps = Regex::new(r".+ map:\n([\s\S]+)").unwrap();

    let seeds: Vec<u64> = seeds_re
        .captures(seeds)
        .unwrap()[1]
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    // let mut maps_vec: Vec<Map> = vec![];
    let mut almanac = Almanac::new();

    for line in lines {
        
        if let Some(cap) = maps.captures(line) {
            // maps_vec.push(Map::from_string(&cap[1]));
            almanac.add(Map::from_string(&cap[1]));
        }
    };

    (seeds, almanac)
}


fn solution_pt1(input: &str) -> u64 {
    let (seeds, maps): (Vec<u64>, Almanac) = parse(input);

    seeds
        .iter()
        .map(|s| maps.get(*s))
        .collect::<Vec<u64>>()
        .iter()
        .reduce(|acc, e| acc.min(e))
        .unwrap()
        .to_owned()
}

fn solution_pt2(input: &str) -> u64 {
    let (seeds, maps): (Vec<u64>, Almanac) = parse(input);
    let mut seed_range: Vec<SliceRange> = vec![];
    let mut min_val = u64::MAX - 1;
    
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let stop = &start + seeds[i + 1];
        // let range = (start..stop);
        let range = SliceRange(start, stop - 1);

        seed_range.push(range)
    }

    // let seed_max = seed_range.iter().map(|r| r.end).reduce(|acc, e| acc.max(e)).unwrap();
    // let seed_min = seed_range.iter().map(|r| r.start).reduce(|acc, e| acc.min(e)).unwrap();

    // println!("{} {}", maps.max_val, maps.min_val);
    // println!("{} {}", seed_max, seed_min);

    for sr in seed_range {
        // for i in sr {
        //     min_val = min_val.min(maps.get(i));
        // }
        min_val = min_val.min(maps.get_from_range(sr));
        
    };
    
    min_val
}

pub fn main() {
    let puzzle = get_puzzle("23", "5");

    time_it!("Part 1", solution_pt1(&puzzle));
    time_it!("Part 2", solution_pt2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &'static str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";


    #[test]
    fn test_parse() {
        let (seeds, maps): (Vec<u64>, Almanac) = parse(TEST);
        assert_eq!(seeds.len(), 4);
        assert_eq!(maps.maps.len(), 7);
        assert_eq!(maps.maps[0].rlist.len(), 2);
        
        assert_eq!(maps.maps[0].get(79), 81);
        assert_eq!(maps.maps[1].get(81), 81);
        assert_eq!(maps.maps[2].get(81), 81);
        assert_eq!(maps.maps[3].get(81), 74);
        assert_eq!(maps.maps[4].get(74), 78);
        assert_eq!(maps.maps[5].get(78), 78);
        assert_eq!(maps.maps[6].get(78), 82);
    }

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_solution2() {
        let res = solution_pt2(TEST);
        assert_eq!(res, 46)
    }




}