use std::{collections::HashSet, ops::{Range, RangeInclusive}};

use phf::{phf_map, Set};

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 4;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    Solution::I32(find_fully_contained(parse_pairs(input)).len() as i32)
}

fn solve2(input: &str) -> Solution {
    Solution::I32(find_partly_contained(parse_pairs(input)).len() as i32)
}

type Pairs = Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>;

fn parse_pairs(input: &str) -> Pairs {
    input.split("\n").into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let ranges = line.split(",").into_iter()
                .map(|range| {
                    let start_end: Vec<_> = range.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
                    start_end[0]..=start_end[1]
                }).collect::<Vec<_>>();
            (ranges[0].clone(), ranges[1].clone())
        }).collect()
}

fn find_fully_contained(pairs: Pairs) -> Pairs {
    pairs.iter()
        .filter(|(left, right)| {
            left.clone().into_iter().all(|l| right.contains(&l)) || right.clone().into_iter().all(|r| left.contains(&r))
        }).cloned().collect()
}

fn find_partly_contained(pairs: Pairs) -> Pairs {
    pairs.iter()
        .filter(|(left, right)| {
            left.clone().into_iter().any(|l| right.contains(&l))
        }).cloned().collect()
}

mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{DAY, solve1, solve2, parse_pairs, find_fully_contained, find_partly_contained};

    fn sample_input() -> String {
        get_input(DAY, true, None)
    }

    #[test]
    fn parse_test() {
        assert_eq!(vec![(1..=2,5..=7)], parse_pairs("1-2,5-7"));
        assert_eq!(vec![
            (1..=2,5..=7),
            (8..=9,8..=11)
        ], parse_pairs("1-2,5-7\n8-9,8-11"));
    }

    #[test]
    fn find_fully_contained_test() {
        assert_eq!(vec![(1..=2, 1..=4)], find_fully_contained(vec![(1..=2, 1..=4), (1..=2, 3..=4)]));
        assert_eq!(vec![(1..=2, 1..=4), (5..=8, 8..=8)], find_fully_contained(vec![(1..=2, 1..=4), (5..=8, 8..=8)]));
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::I32(2), solve1(&sample_input()))
    }

    #[test]
    fn find_partly_contained_test() {
        assert_eq!(vec![(5..=7, 7..=9)], find_partly_contained(vec![(5..=7, 7..=9), (1..=2, 3..=5)]));
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(4), solve2(&sample_input()));
    }
}