
use std::env;
use std::collections::HashMap;
// use std::process::Command;

pub mod year2023 {
    pub mod day1;
    pub mod day2;
    pub mod day23;
}

pub mod utils {
    pub mod grid;
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        {
            use $year::$day::main as solution_main;
            solution_main as fn()
        }
    };
}

fn y2023(day: &str) -> () {
    let solutions_map = HashMap::from([
        ("1", solution!(year2023, day1)),
        ("2", solution!(year2023, day2)),
        ("23", solution!(year2023, day23))
    ]);

    // let solutions = vec![
    //     solution!(year2023, day1),
    //     solution!(year2023, day23)
    // ];

    solutions_map.get(day).expect("Not in solutions map")();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need more args")
    };
    let day: &String = &args[1];
    println!("{} {}", "Running Day", &day);
    
    // solution!(year2023, day1)
    y2023(&day)

}

