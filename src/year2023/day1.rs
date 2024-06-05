use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Instant;

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solution_pt1(data: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;

    for line in data {
        let mut number: Vec<char> = vec![];

        for letter in line.chars() {
            if letter.is_numeric() {
                number.push(letter);
            }
        }

        if !number.is_empty() {
            let first_last = format!("{}{}", number[0], number[number.len() - 1]);
            result += first_last.parse::<u32>().unwrap();
        }
    }

    result
}

fn solution_pt2(data: &Vec<&str>) -> u32 {
    let word_to_num: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut result: u32 = 0;

    for line in data {
        let mut cur_line = line.to_owned();

        let first = 'outer: loop {
            let l: Vec<char> = cur_line.chars().collect();
            if l[0].is_numeric() {
                break l[0].to_string();
            }

            for key in word_to_num.keys() {
                if cur_line.starts_with(key) {
                    break 'outer word_to_num.get(key).unwrap().to_string();
                }
            }
            cur_line = &cur_line[1..];
        };

        let last = 'outer: loop {
            let l: Vec<char> = cur_line.chars().collect();
            if l[l.len() - 1].is_numeric() {
                break l[l.len() - 1].to_string();
            }

            for key in word_to_num.keys() {
                if cur_line.ends_with(key) {
                    break 'outer word_to_num.get(key).unwrap().to_string();
                }
            }

            cur_line = &cur_line[..cur_line.len() - 1];
        };

        result += format!("{}{}", first, last).parse::<u32>().unwrap();
    }

    result
}

pub fn main() {
    let path: PathBuf = ["input", "y23", "day1.txt"].iter().collect();
    let data = read_to_string(path).expect("Not there");
    let data_parsed = parse(&data);

    let start = Instant::now();
    let res = solution_pt1(&data_parsed);
    let res2: u32 = solution_pt2(&data_parsed);
    let duration = start.elapsed().as_micros();
    println!("{res} {res2} {duration}")
}

