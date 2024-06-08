#![allow(unused)]

use clap::Parser;
use paste::paste;
use std::collections::HashMap;
use std::env;
use std::fs::{self, read_to_string};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Year to work with
    #[arg(short, long, default_value = "2023")]
    year: u32,

    /// Target day
    day: String,
}

macro_rules! aoc {
    () => {};

    ( $( $year:literal => [ $($days:literal $(,)?)* ] $(,)? )* ) => {
        paste! {
            $(
                pub mod [<year$year>]  {
                    $(
                        pub mod [<day$days>];
                    )*
                }

                fn [<y$year>](day: &str) {
                    let solutions_map = ::std::collections::HashMap::from([
                        $(
                            (
                                stringify!($days),
                                {
                                    use [<year$year>]::[<day$days>]::main as solution_main;
                                    solution_main as fn()
                                }
                            )
                        ),*
                    ]);

                    solutions_map.get(day).expect("Not in solutions map")();
                }
            )*
        }
    };
}

aoc!(
    2022 => [1, 2, 3],
    2023 => [1, 2, 3, 4, 5, 6, 7, 8, 10, 23]
);

#[macro_export]
macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        let res = { $s };
        println!("{}: {:?}\n{:?}", $context, timer.elapsed(), res);
    };
}

pub fn debug_output<O: AsRef<str>>(output: O, year: usize, day: usize) {
    let path = PathBuf::from_iter(["debug", &format!("y{year}/day{day}.txt")]);
    fs::write(path, output.as_ref()).unwrap();
}

pub fn get_puzzle(year: &str, day: &str) -> String {
    let path = PathBuf::from_iter(["input", &format!("y{year}/day{day}.txt")]);
    read_to_string(path).expect("Not there")
}

fn main() {
    let args = Args::parse();

    let year = match args.year {
        2023 | 23 => y2023,
        2022 | 22 => y2022,
        y => panic!("Invalid year {}", y),
    };

    println!("Running Day: {}, Year: {}", &args.day, args.year);

    year(&args.day);
}
