use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{debug_output_logger, get_puzzle, time_it};
use log::info;
use num_integer::Integer;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High = 1,
    Low = 0,
}

#[derive(Debug, Clone, Copy)]
enum Status {
    On = 1,
    Off = 0,
}

#[derive(Debug, Clone)]
enum Modules<'b> {
    Broadcast(Vec<&'b str>),
    FlipFlop(Vec<&'b str>, Status),
    Junction(Conjunction<'b>),
    Output,
}

#[derive(Debug, Clone, Default)]
struct Conjunction<'b> {
    input: HashMap<&'b str, Pulse>,
    output: Vec<&'b str>,
    count: usize,
}

#[derive(Debug, Clone)]
struct Broadcaster<'b> {
    modules: HashMap<&'b str, Modules<'b>>,
    report: Vec<&'b str>,
}

#[derive(Debug, Clone)]
struct Instructions<'b>(&'b str, &'b str, Pulse);

impl<'b> Broadcaster<'b> {
    fn start(&mut self) -> (usize, usize) {
        use Modules::*;
        let mut queue = VecDeque::new();
        let (mut high, mut low) = (0, 1);

        self.report.clear();

        if let Some(Broadcast(b)) = self.modules.get("broadcaster") {
            queue.extend(b.iter().map(|m| Instructions("broadcaster", m, Pulse::Low)));
        } else {
            panic!("Should not happen")
        }

        while let Some(Instructions(from, to, pulse)) = queue.pop_front() {
            // info!("From - {}, To - {}, Pulse - {:?}", from, to, pulse);

            if let ("xn", Pulse::High) = (to, pulse) {
                self.report.push(from);
            }

            if let Some(module) = self.modules.get_mut(to) {
                match module {
                    FlipFlop(v, stat) if pulse == Pulse::Low => {
                        *stat = stat.flip();

                        match stat {
                            Status::On => {
                                queue.extend(v.iter().map(|m| Instructions(to, m, Pulse::High)))
                            }
                            Status::Off => {
                                queue.extend(v.iter().map(|m| Instructions(to, m, Pulse::Low)))
                            }
                        };
                    }
                    Junction(conj) => {
                        conj.flip_mod(from, pulse);
                        let pulse = conj.pulse();

                        queue.extend(conj.output.iter().map(|o| Instructions(to, o, pulse)));
                    }
                    _ => {}
                }
            }

            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            };
        }

        (high, low)
    }
}

impl Pulse {
    fn flip(self) -> Self {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

impl Status {
    fn flip(self) -> Self {
        match self {
            Status::On => Status::Off,
            Status::Off => Status::On,
        }
    }
}

impl<'b> Conjunction<'b> {
    fn flip_mod(&mut self, entry: &'b str, pulse: Pulse) {
        self.input.entry(entry).and_modify(|e| *e = pulse);
    }

    fn pulse(&self) -> Pulse {
        if self.input.values().all(|v| *v as u8 == 1) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

fn parse(input: &str) -> Broadcaster<'_> {
    let regex = Regex::new(r"(.+) -> (.+)\n").unwrap();
    let mut modules = HashMap::new();
    let mut modules_to_check = HashSet::new();
    modules.insert("output", Modules::Output);

    for (_, [key, value]) in regex.captures_iter(input).map(|c| c.extract()) {
        let value: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

        match key {
            "broadcaster" => {
                modules_to_check.extend(value.iter().cloned().map(|v| ("broadcaster", v)));
                modules.insert("broadcaster", Modules::Broadcast(value));
            }
            flip if flip.starts_with('%') => {
                modules_to_check.extend(value.iter().cloned().map(|v| (&flip[1..], v)));
                modules.insert(&flip[1..], Modules::FlipFlop(value, Status::Off));
            }
            inv if inv.starts_with('&') => {
                modules_to_check.extend(value.iter().cloned().map(|v| (&inv[1..], v)));
                modules.insert(
                    &inv[1..],
                    Modules::Junction(Conjunction {
                        output: value,
                        ..Default::default()
                    }),
                );
            }
            _ => {}
        };
    }

    for (from, to) in modules_to_check {
        if let Some(Modules::Junction(conj)) = modules.get_mut(to) {
            conj.input.insert(from, Pulse::Low);
        }
    }

    Broadcaster {
        modules,
        report: vec![],
    }
}

fn solution_pt1(input: &str, tries: usize) -> usize {
    let (mut high, mut low) = (0, 0);
    let mut broadcaster = parse(input);

    for i in 0..tries {
        let (h, l) = broadcaster.start();
        high += h;
        low += l;
    }

    high * low
}

fn solution_pt2(input: &str) -> usize {
    let mut broadcaster = parse(input);
    let mut targets = if let Some(Modules::Junction(conj)) = broadcaster.modules.get("xn") {
        conj.input.clone()
    } else {
        panic!("Should exist")
    };
    let mut count = 0;
    let mut cycles = vec![];

    while !targets.is_empty() {
        count += 1;
        let _ = broadcaster.start();

        for r in broadcaster.report.iter() {
            cycles.push(count);
            let _ = targets.remove(r);
        }
    }

    cycles.iter().fold(1, |acc, num| acc.lcm(num))
}

pub fn main() {
    let puzzle = get_puzzle("23", "20");

    time_it!("Solution Pt 1", solution_pt1(&puzzle, 1000));
    time_it!("Solution Pt 2", solution_pt2(&puzzle));
}

#[cfg(test)]
const TEST_ONE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

// #[cfg(test)]
const TEST_TWO: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let res = solution_pt1(TEST_ONE, 1);

        assert_eq!(res, 32);
    }

    #[test]
    fn test_solution_1_1000() {
        let res = solution_pt1(TEST_ONE, 1000);

        assert_eq!(res, 32000000);
    }

    #[test]
    fn test_solution_1_2() {
        let res = solution_pt1(TEST_TWO, 1000);

        assert_eq!(res, 11687500);
    }
}
