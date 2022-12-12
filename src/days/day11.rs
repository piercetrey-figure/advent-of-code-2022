use std::collections::HashSet;

use phf::{phf_map, Set};
use regex::Regex;

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 11;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let monkeys = parse_input(input);
    Solution::I32(0)
}

fn solve2(input: &str) -> Solution {
    Solution::I32(0)
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkeys = input.split("\n\n").map(|monkey| {
        let regex = Regex::new(r"Monkey (?P<index>[0-9]+):.*
\s*Starting items: (?P<items>[0-9, ]+).*
\s*Operation: new = (?P<operation>.*).*
\s*Test: divisible by (?P<divisor>[0-9]+).*
\s*  If true: throw to monkey (?P<true_monkey>[0-9]+).*
\s*  If false: throw to monkey (?P<false_monkey>[0-9]+).*").unwrap();
  let matches = regex.captures(monkey).unwrap();
  
        Monkey { items: matches.name("items").map(|items| items.as_str().split(", ").map(|item| item.parse().unwrap()).collect()).unwrap(), operation: |old: i32| {
            matches.name("operation").map(|operation| {
                let portions: Vec<_> = operation.as_str().split(" ").collect();
                let operator = portions[1];
                match operator {
                    "*" => {

                    },
                    _ => panic!("Unknown operator {}", operator)
                }
            })
        }, test: () }
    }).collect();
    monkeys
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: fn(i32) -> i32,
    test: fn(i32) -> i32,
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{DAY, solve1, solve2};

    fn sample_input() -> String {
        get_input(DAY, true, None)
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::I32(10605), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(0), solve2(&sample_input()));
    }
}