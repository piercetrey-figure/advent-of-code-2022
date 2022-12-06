use std::collections::HashMap;

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 5;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    solver(input, false)
}

fn solve2(input: &str) -> Solution {
    solver(input, true)
}

fn solver(input: &str, at_once: bool) -> Solution {
    let (mut stack, mut moves) = parse_input(input);
    loop {
        (stack, moves) = advance_stack((stack.clone(), moves.clone()), at_once);
        if moves.is_empty() {
            return Solution::String(code(stack));
        }
    }
}

type Stack = HashMap<i32, Vec<String>>;
type Move = (i32, i32, i32);
type Moves = Vec<Move>;

fn advance_stack((mut stack, moves): (Stack, Moves), at_once: bool) -> (Stack, Moves) {
    assert!(
        !moves.is_empty(),
        "Cannot advance a stack with no moves, bro"
    );
    let (number, from, to) = moves.first().unwrap();

    let source_stack = stack.get(from).unwrap_or_else(|| {
        panic!("unwrapped failed, ({} {} {})", number, from, to);
    });
    let (mut to_move, rest) = (
        source_stack[0..*number as usize].to_vec(),
        source_stack[*number as usize..source_stack.len()].to_vec(),
    );
    if at_once {
        to_move.reverse();
    }
    stack.insert(*from, rest);
    for e in to_move {
        stack.entry(*to).and_modify(|entry| entry.insert(0, e));
    }

    (stack, moves[1..moves.len()].to_vec())
}

fn parse_input(input: &str) -> (Stack, Moves) {
    let parts = input
        .split("\n\n")
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    (parse_stack(parts[0]), parse_moves(parts[1]))
}

fn parse_stack(input: &str) -> Stack {
    input
        .split("\n")
        .filter(|l| l.contains("["))
        .fold(
            &mut HashMap::new(),
            |acc: &mut HashMap<i32, Vec<String>>, line| {
                let line = line
                    .char_indices()
                    .filter(|(i, _)| i % 4 == 1)
                    .map(|(_, c)| c)
                    .collect::<Vec<_>>()
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (i + 1, c.to_string()))
                    .filter(|(_, c)| !c.trim().is_empty())
                    .collect::<Vec<_>>();

                for (i, val) in line {
                    acc.entry(i as i32)
                        .and_modify(|e| e.push(val.to_string()))
                        .or_insert(vec![val.to_string()]);
                }
                acc
            },
        )
        .to_owned()
}

fn parse_moves(input: &str) -> Moves {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums: Vec<i32> = l
                .split(" ")
                .filter(|segment| !vec!["move", "from", "to"].contains(segment))
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn code(stack: Stack) -> String {
    let mut stack_v = stack.iter().collect::<Vec<_>>();
    stack_v.sort_by_key(|(k, _)| **k);
    stack_v
        .iter()
        .map(|(_, v)| v.first().map(|s| s.to_string()).unwrap_or("".to_string()))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{input::get_input, solution::Solution};

    use super::{advance_stack, code, parse_input, solve1, solve2, DAY};

    fn sample_input() -> String {
        get_input(DAY, true, None)
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            (
                HashMap::from([
                    (1i32, vec!["N".to_string(), "Z".to_string()]),
                    (
                        2i32,
                        vec!["D".to_string(), "C".to_string(), "M".to_string()]
                    ),
                    (3i32, vec!["P".to_string()])
                ]),
                vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2),]
            ),
            parse_input(&sample_input())
        )
    }

    #[test]
    fn test_advance() {
        let (stack, moves) = (
            HashMap::from([
                (1i32, vec!["N".to_string(), "Z".to_string()]),
                (
                    2i32,
                    vec!["D".to_string(), "C".to_string(), "M".to_string()],
                ),
                (3i32, vec!["P".to_string()]),
            ]),
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
        );
        assert_eq!(
            (
                HashMap::from([
                    (
                        1i32,
                        vec!["D".to_string(), "N".to_string(), "Z".to_string()]
                    ),
                    (2i32, vec!["C".to_string(), "M".to_string()],),
                    (3i32, vec!["P".to_string()]),
                ]),
                vec![(3, 1, 3), (2, 2, 1), (1, 1, 2)],
            ),
            advance_stack((stack, moves), false)
        );
    }

    #[test]
    fn test_advance_at_once() {
        let (stack, moves) = (
            HashMap::from([
                (1i32, vec!["N".to_string(), "Z".to_string()]),
                (
                    2i32,
                    vec!["D".to_string(), "C".to_string(), "M".to_string()],
                ),
                (3i32, vec!["P".to_string()]),
            ]),
            vec![(3, 2, 1), (2, 2, 1), (1, 1, 2)],
        );
        assert_eq!(
            (
                HashMap::from([
                    (
                        1i32,
                        vec![
                            "D".to_string(),
                            "C".to_string(),
                            "M".to_string(),
                            "N".to_string(),
                            "Z".to_string()
                        ]
                    ),
                    (2i32, vec![],),
                    (3i32, vec!["P".to_string()]),
                ]),
                vec![(2, 2, 1), (1, 1, 2)],
            ),
            advance_stack((stack, moves), true)
        );
    }

    #[test]
    fn stack_code_test() {
        assert_eq!(
            "NDP",
            code(HashMap::from([
                (1i32, vec!["N".to_string(), "Z".to_string()]),
                (
                    2i32,
                    vec!["D".to_string(), "C".to_string(), "M".to_string()],
                ),
                (3i32, vec!["P".to_string()]),
            ]))
        );
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::String("CMZ".into()), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::String("MCD".into()), solve2(&sample_input()));
    }
}
