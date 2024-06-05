#![allow(unused)]

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
// use std::process::Command;

pub mod year2022 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}

pub mod year2023 {
    pub mod day1;
    pub mod day2;
    pub mod day23;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
}

pub mod utils {
    pub mod grid;
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        use $year::$day::main as solution_main;
        solution_main as fn()
    }};
}

#[macro_export]
macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        let res = { $s };
        println!("{}: {:?}\n{:?}", $context, timer.elapsed(), res);
    };
}

fn y2022(day: &str) {
    let solutions_map = HashMap::from([
        ("1", solution!(year2022, day1)),
        ("2", solution!(year2022, day2)),
        ("3", solution!(year2022, day3)),
    ]);

    solutions_map.get(day).expect("Not in solutions map")();
}

fn y2023(day: &str) {
    let solutions_map = HashMap::from([
        ("1", solution!(year2023, day1)),
        ("2", solution!(year2023, day2)),
        ("3", solution!(year2023, day3)),
        ("4", solution!(year2023, day4)),
        ("5", solution!(year2023, day5)),
        ("6", solution!(year2023, day6)),
        ("7", solution!(year2023, day7)),
        ("23", solution!(year2023, day23)),
    ]);

    solutions_map.get(day).expect("Not in solutions map")();
}

pub fn get_puzzle(year: &str, day: &str) -> String {
    let path: PathBuf = [
        "input",
        format!("y{}", year).as_str(),
        format!("day{}.txt", day).as_str(),
    ]
    .iter()
    .collect();
    read_to_string(path).expect("Not there")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need more args")
    };
    let day: &String = &args[1];
    println!("Running day {}", &day);

    // solution!(year2023, day1)
    y2023(day)
    // y2022(&day)
}
