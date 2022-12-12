use std::{
    collections::VecDeque,
    ops::{Add, Div, Mul, Sub},
};

use regex::Regex;
use rug::{ops::DivRounding, Assign, Integer};

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
    do_the_monkey_business(input, 20, true).into()
}

fn solve2(input: &str) -> Solution {
    do_the_monkey_business(input, 10000, false).into()
}

fn do_the_monkey_business(input: &str, num_rounds: i32, worry_divide: bool) -> Integer {
    let worry_divisor: Integer = 3u8.into();
    let mut monkeys = parse_input(input);
    let mut inspection_counter: Vec<u64> = monkeys.iter().map(|_| 0).collect();
    let magic_number: &Integer = &monkeys.iter().map(|m| m.test_details.0.clone()).product();
    for _ in 0..num_rounds {
        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys.get_mut(monkey_index).unwrap().items.pop_front() {
                inspection_counter[monkey_index] += 1;
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let mut new_worry_level = (monkey.operation)(monkey, item);
                if worry_divide {
                    new_worry_level = new_worry_level.div_floor(&worry_divisor);
                }
                new_worry_level %= magic_number;
                let destination_monkey = (monkey.test)(&monkey, &new_worry_level);
                monkeys
                    .get_mut(destination_monkey as usize)
                    .unwrap()
                    .items
                    .push_back(new_worry_level);
            }
        }
    }
    inspection_counter.sort();
    (inspection_counter[inspection_counter.len() - 2]
        * inspection_counter[inspection_counter.len() - 1])
        .into()
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let regex = Regex::new(
                r"Monkey (?P<index>[0-9]+):.*
\s*Starting items: (?P<items>[0-9, ]+).*
\s*Operation: new = (?P<operation>.*).*
\s*Test: divisible by (?P<divisor>[0-9]+).*
\s*  If true: throw to monkey (?P<true_monkey>[0-9]+).*
\s*  If false: throw to monkey (?P<false_monkey>[0-9]+).*",
            )
            .unwrap();
            let matches = regex.captures(monkey).unwrap();
            let operation_str = matches
                .name("operation")
                .map(|operation| operation.as_str().to_string())
                .unwrap();
            let test_details = (
                matches.name("divisor").unwrap().as_str().parse().unwrap(),
                matches
                    .name("true_monkey")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                matches
                    .name("false_monkey")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            );
            Monkey {
                items: matches
                    .name("items")
                    .map(|items| {
                        items
                            .as_str()
                            .split(", ")
                            .map(|item| item.parse().unwrap())
                            .collect()
                    })
                    .unwrap(),
                operation_str,
                operation_details: None,
                operation: |monkey: &mut Monkey, old: Integer| {
                    let mut val1: Operand;
                    let mut val2: Operand;
                    let mut operator: Operator;
                    match &monkey.operation_details {
                        Some(operation_details) => {
                            val1 = operation_details.0.clone();
                            operator = operation_details.1.clone();
                            val2 = operation_details.2.clone();
                        }
                        None => {
                            let portions: Vec<_> = monkey.operation_str.split(" ").collect();
                            let operator_str = portions[1];
                            val1 = if portions[0] == "old" {
                                Operand::Old
                            } else {
                                Operand::Val(portions[0].parse::<u32>().unwrap().into())
                            };
                            val2 = if portions[2] == "old" {
                                Operand::Old
                            } else {
                                Operand::Val(portions[2].parse::<u32>().unwrap().into())
                            };
                            operator = match operator_str {
                                "*" => Operator::Mul,
                                "/" => Operator::Div,
                                "+" => Operator::Add,
                                "-" => Operator::Sub,
                                _ => panic!("Unknown operator {}", operator_str),
                            };
                            monkey.operation_details =
                                Some((val1.clone(), operator.clone(), val2.clone()));
                        }
                    };
                    let val1 = match val1 {
                        Operand::Old => old.clone(),
                        Operand::Val(v) => v,
                    };
                    let val2 = match val2 {
                        Operand::Old => old,
                        Operand::Val(v) => v,
                    };
                    match operator {
                        Operator::Mul => val1 * val2,
                        Operator::Div => val1 / val2,
                        Operator::Add => val1 + val2,
                        Operator::Sub => val1 - val2,
                    }
                },
                test_details,
                test: |monkey: &Monkey, value: &Integer| {
                    let (divisor, true_monkey, false_monkey) = &monkey.test_details;
                    if value % divisor.clone() == Integer::ZERO {
                        *true_monkey
                    } else {
                        *false_monkey
                    }
                },
            }
        })
        .collect();
    monkeys
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<Integer>,
    operation_str: String,
    operation_details: Option<(Operand, Operator, Operand)>,
    operation: fn(&mut Monkey, Integer) -> Integer,
    test_details: (Integer, i32, i32),
    test: fn(&Monkey, &Integer) -> i32,
}

#[derive(Clone)]
enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone)]
enum Operand {
    Old,
    Val(Integer),
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{solve1, solve2, DAY};

    fn sample_input() -> String {
        get_input(DAY, true, None)
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::Integer(10605u32.into()), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            Solution::Integer(2713310158u32.into()),
            solve2(&sample_input())
        );
    }
}
