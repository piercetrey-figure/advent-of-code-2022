use std::collections::VecDeque;

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
    do_the_monkey_business(input, 20, true).into()
}

fn solve2(input: &str) -> Solution {
    do_the_monkey_business(input, 10000, false).into()
}

fn do_the_monkey_business(input: &str, num_rounds: i32, worry_divide: bool) -> u64 {
    let mut monkeys = parse_input(input);
    let mut inspection_counter: Vec<u64> = monkeys.iter().map(|_| 0).collect();
    let magic_number: u64 = monkeys.iter().map(|m| m.test_details.0).product();
    for _ in 0..num_rounds {
        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys.get_mut(monkey_index).unwrap().items.pop_front() {
                inspection_counter[monkey_index] += 1;
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let mut new_worry_level = (monkey.operation)(monkey, item);
                if worry_divide {
                    new_worry_level = new_worry_level.div_floor(3);
                }
                new_worry_level %= magic_number;
                let destination_monkey = (monkey.test)(&monkey, new_worry_level);
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
                operation: |monkey: &mut Monkey, old: u64| {
                    let (val1, operator, val2) = match &monkey.operation_details {
                        Some(operation_details) => *operation_details,
                        None => {
                            let portions: Vec<_> = monkey.operation_str.split(" ").collect();
                            let operator_str = portions[1];
                            let val1 = if portions[0] == "old" {
                                Operand::Old
                            } else {
                                Operand::Val(portions[0].parse::<u64>().unwrap())
                            };
                            let val2 = if portions[2] == "old" {
                                Operand::Old
                            } else {
                                Operand::Val(portions[2].parse::<u64>().unwrap())
                            };
                            let operator = match operator_str {
                                "*" => Operator::Mul,
                                "/" => Operator::Div,
                                "+" => Operator::Add,
                                "-" => Operator::Sub,
                                _ => panic!("Unknown operator {}", operator_str),
                            };
                            monkey.operation_details = Some((val1, operator, val2));
                            (val1, operator, val2)
                        }
                    };

                    let val1 = match val1 {
                        Operand::Old => old,
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
                test: |monkey: &Monkey, value: u64| {
                    let (divisor, true_monkey, false_monkey) = &monkey.test_details;
                    if value % divisor == 0 {
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
    items: VecDeque<u64>,
    operation_str: String,
    operation_details: Option<(Operand, Operator, Operand)>,
    operation: fn(&mut Monkey, u64) -> u64,
    test_details: (u64, i32, i32),
    test: fn(&Monkey, u64) -> i32,
}

#[derive(Clone, Copy)]
enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Copy)]
enum Operand {
    Old,
    Val(u64),
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
        assert_eq!(Solution::U64(10605), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::U64(2713310158), solve2(&sample_input()));
    }
}
