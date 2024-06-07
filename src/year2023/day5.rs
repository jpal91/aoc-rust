use cached::proc_macro::cached;
use cached::{Cached, SizedCache};
use regex::Regex;
use std::{
    collections::HashMap,
    ops::{Deref, Range},
};

use crate::{get_puzzle, time_it};

#[derive(Debug)]
struct Map {
    rlist: Vec<(Range<usize>, usize)>,
    min: usize,
    max: usize,
}

impl Map {
    fn get(&self, val: usize) -> usize {
        if val < self.min || val >= self.max {
            return val;
        }

        for (range, base) in self.rlist.iter() {
            if range.contains(&val) {
                return (val - range.start) + base;
            }
        }

        val
    }

    fn from_string(input: &str) -> Self {
        let mut rlist = vec![];
        let mut min = usize::MAX - 1;
        let mut max = 0;

        for line in input.lines() {
            let nums: Vec<usize> = line
                .split(' ')
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect();

            let (start, index, base) = (nums[0], nums[1], nums[2]);
            let map_range = (index..index + base, start);
            min = min.min(index);
            max = max.max(index + base);

            rlist.push(map_range);
        }

        Self { rlist, min, max }
    }

    fn intersect(self, other: Map) -> Map {
        let mut new_ranges: Vec<(Range<usize>, usize)> = vec![];

        for (range, _) in self.rlist {
            for (o_range, o_dest) in other.rlist.iter() {
                if range.start < o_range.end && range.end > o_range.start {
                    let start = range.start.max(o_range.start);
                    let end = range.end.min(o_range.end);
                    let new_start = (start - o_range.start) + o_dest;
                    let new_end = o_dest + (end - o_range.start);
                    new_ranges.push((new_start..new_end, 0));
                }
            }
        }

        let mut other = other
            .rlist
            .into_iter()
            .map(|(range, dest)| (dest..dest + (range.end - range.start), 0))
            .collect::<Vec<_>>();
        other.extend(new_ranges);
        other.sort_by(|a, b| a.0.start.cmp(&b.0.start));
        println!("{:?}", other);

        let mut current = other[0].0.clone();
        let mut rlist: Vec<(Range<usize>, usize)> = vec![];

        for (range, _) in other[1..].iter() {
            if current.end < range.start {
                rlist.push((current, 0));
                current = range.clone();
            } else {
                current.end = current.end.max(range.end);
            }
        }

        rlist.push((current, 0));

        Self {
            rlist,
            min: 0,
            max: 0,
        }
    }
}

fn check_almanac(
    maps: &[Map],
    cache: &mut HashMap<(usize, usize), usize>,
    level: usize,
    old_val: usize,
) -> usize {
    // println!("val - {} level - {}", val, level);
    if let Some(&cached_val) = cache.get(&(level, old_val)) {
        return cached_val;
    }
    // println!("old val - {}", old_val);

    let val = maps[level].get(old_val);

    // println!("val - {} level - {}", val, level);
    if level == 6 {
        // println!("last val - {}", val);
        return val;
    }

    let res = check_almanac(maps, cache, level + 1, val);
    cache.insert((level, val), res);

    res
}

#[cached(
    ty = "SizedCache<(usize, usize), usize>",
    create = "{ SizedCache::with_size(100000) }",
    convert = "{ (level, old_val) }"
)]
fn check_almanac_cached(
    maps: &[Map],
    // cache: &mut HashMap<(usize, usize), usize>,
    level: usize,
    old_val: usize,
) -> usize {
    let val = maps[level].get(old_val);

    if level == 6 {
        return val;
    }

    check_almanac_cached(maps, level + 1, val)
}

fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.split("\n\n");
    let seeds = lines.next().unwrap();

    let seeds_re = Regex::new(r"seeds: (.+)").unwrap();
    let maps = Regex::new(r".+ map:\n([\s\S]+)").unwrap();

    let seeds: Vec<u64> = seeds_re.captures(seeds).unwrap()[1]
        .split(' ')
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    // let mut maps_vec: Vec<Map> = vec![];
    let mut almanac: Vec<Map> = vec![];

    for line in lines {
        if let Some(cap) = maps.captures(line) {
            // maps_vec.push(Map::from_string(&cap[1]));
            almanac.push(Map::from_string(&cap[1]));
        }
    }

    (seeds, almanac)
}

fn solution_pt1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    let mut _cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut min = usize::MAX - 1;

    for seed in seeds {
        // let res = check_almanac(&maps, &mut cache, 0, seed as usize);
        let res = check_almanac_cached(&maps, 0, seed as usize);
        // println!("seed - {} res - {}", seed, res);
        min = min.min(res);
    }

    println!(
        "{}",
        CHECK_ALMANAC_CACHED
            .lock()
            .unwrap()
            .cache_hits()
            .unwrap_or_default()
    );

    min
}

fn solution_pt2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    let mut min = usize::MAX - 1;

    // for se in seeds.chunks(2) {
    //     let (start, end) = (se[0], se[0] + se[1]);
    //     for seed in start..end {
    //         let res = check_almanac_cached(&maps, 0, seed as usize);
    //         min = min.min(res);
    //     }
    // }
    //
    // println!(
    //     "{}",
    //     CHECK_ALMANAC_CACHED
    //         .lock()
    //         .unwrap()
    //         .cache_hits()
    //         .unwrap_or_default()
    // );

    let seed_maps = seeds
        .chunks(2)
        .map(|c| (c[0] as usize..(c[0] + c[1]) as usize, 0))
        .collect::<Vec<_>>();

    let mut current_map = Map {
        rlist: seed_maps,
        min: 0,
        max: 0,
    };

    for map in maps {
        current_map = current_map.intersect(map);
    }

    println!("{:?}", current_map);

    min
}

pub fn main() {
    let input = get_puzzle("23", "5");

    time_it!("Solution Pt 1", solution_pt1(&input));
    time_it!("Solution Pt 2", solution_pt2(&input));
}

#[cfg(test)]
const TEST_MAP: &str = "\
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed = parse(TEST_MAP);

        println!("{:?}\n\n{:?}", parsed.0, parsed.1);
    }

    #[test]
    fn test_maps() {
        let (_, maps) = parse(TEST_MAP);
        assert_eq!(maps.len(), 7);

        let start_expect = [(79, 82), (14, 43)];

        for (s, e) in start_expect {
            let mut val = s;

            for map in maps.iter() {
                println!("v - {} ", val);
                val = map.get(val);
            }

            assert_eq!(val, e);
        }
    }

    #[test]
    fn test_pt1() {
        let res = solution_pt1(TEST_MAP);

        assert_eq!(res, 35);
    }

    #[test]
    fn test_pt2() {
        let res = solution_pt2(TEST_MAP);

        assert_eq!(res, 46);
    }

    #[test]
    fn test_intersect() {
        let map1 = Map {
            rlist: vec![(79..93, 0), (55..68, 0)],
            min: 0,
            max: 0,
        };
        let map2 = Map {
            rlist: vec![(98..100, 50), (50..98, 52)],
            min: 0,
            max: 0,
        };

        let res = map1.intersect(map2);

        println!("{:?}", res);

        let map3 = Map {
            rlist: vec![(15..52, 0), (52..54, 37), (0..15, 39)],
            min: 0,
            max: 0,
        };

        let res = res.intersect(map3);

        println!("{:?}", res);
    }
}
