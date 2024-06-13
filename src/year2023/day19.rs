use std::collections::HashMap;

use regex::Regex;

use crate::{get_puzzle, time_it};

macro_rules! parse_op {
    () => {};

    (@op $target:expr, $op:expr, $nums:expr, $parts:expr ) => {
        {
            let target = match $target {
                "x" => $parts.0,
                "m" => $parts.1,
                "a" => $parts.2,
                 _=> $parts.3
            };
            match $op {
                ">" => target > $nums,
                 _=> target < $nums,
            }
        }
    };

    ($target:expr, $op:expr, $nums:expr, $then:expr ) => {
        move |parts: &Parts|  parse_op!(@op $target, $op, $nums, parts).then_some($then)
    };

}

#[derive(Debug, Clone)]
struct Parts(u32, u32, u32, u32);

type Workflow<'w> = HashMap<&'w str, Job<'w>>;
type JobTest<'w> = Box<dyn Fn(&'_ Parts) -> Option<&'w str> + 'w>;

struct Job<'w> {
    jobs: Vec<JobTest<'w>>,
    end: &'w str,
}

struct Machine<'w> {
    flow: Workflow<'w>,
    parts: Vec<Parts>,
}

impl Machine<'_> {
    fn run(&self) -> u32 {
        let mut total_score = 0;

        for part in self.parts.iter() {
            let mut current = "in";
            let mut score = 0;

            loop {
                let job = self.flow.get(current).unwrap();

                match job.test(part) {
                    "A" => {
                        score += part.total();
                        break;
                    }
                    "R" => break,
                    next => current = next,
                }
            }

            total_score += score;
        }

        total_score
    }
}

impl<'w> Job<'w> {
    fn test(&self, parts: &'_ Parts) -> &'w str {
        for job in self.jobs.iter() {
            if let Some(res) = (job)(parts) {
                return res;
            }
        }

        self.end
    }
}

impl Parts {
    fn total(&self) -> u32 {
        self.0 + self.1 + self.2 + self.3
    }
}

fn parse<'w>(input: &'w str) -> Machine<'w> {
    let work_regex = Regex::new(r"(\w+)\{(.+),(\w+)\}\n").unwrap();
    let inner_regex = Regex::new(r"(\w+)(<|>)(\d+):(\w+),?").unwrap();
    let parts_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}\n").unwrap();

    let mut flow = Workflow::new();
    let mut parts = vec![];

    for (_, [outer, inner_op, last]) in work_regex.captures_iter(input).map(|c| c.extract()) {
        // println!("{} {} {}", outer, last, inner_op);
        let mut jobs_vec: Vec<JobTest<'w>> = vec![];

        for (_, [first, op, digit, target]) in
            inner_regex.captures_iter(inner_op).map(|c| c.extract())
        {
            jobs_vec.push(Box::new(parse_op!(
                first,
                op,
                digit.parse::<u32>().unwrap(),
                target
            )));
        }

        let job = Job {
            jobs: jobs_vec,
            end: last,
        };

        flow.insert(outer, job);
    }

    for (_, [x, m, a, s]) in parts_regex.captures_iter(input).map(|c| c.extract()) {
        let (x, m, a, s) = (
            x.parse().unwrap(),
            m.parse().unwrap(),
            a.parse().unwrap(),
            s.parse().unwrap(),
        );

        parts.push(Parts(x, m, a, s));
    }

    Machine { flow, parts }
}

fn solution_pt1(input: &str) -> u32 {
    let machine = parse(input);

    machine.run()
}

pub fn main() {
    let puzzle = get_puzzle("23", "19");

    time_it!("Solution Pt 1", solution_pt1(&puzzle));
    // time_it!("solution Pt 2", solution_pt2(&puzzle));
}

#[cfg(test)]
const TEST_ONE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse(TEST_ONE);
    }

    #[test]
    fn test_macro() {
        let target = "a";
        // let op = parse_op!(target, 1, 0, "some");
        // let op = parse_op!(target, "<", 1, "some"; "x", ">", 4, "elif"; else "else");
        let op = parse_op!(target, "<", 1, "some");
    }

    #[test]
    fn test_one() {
        let res = solution_pt1(TEST_ONE);

        assert_eq!(res, 19114)
    }
}
