use std::collections::HashSet;

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 6;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    Solution::I32(index_of_first_n_unique::<4>(&parse_input(input)))
}

fn solve2(input: &str) -> Solution {
    Solution::I32(index_of_first_n_unique::<14>(&parse_input(input)))
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn index_of_first_n_unique<const N: usize>(chars: &[char]) -> i32 {
    chars
        .array_windows::<N>()
        .enumerate()
        .find(|(_, w)| HashSet::from(**w).len() == N)
        .map(|(i, _)| i + N)
        .unwrap() as i32
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{solve1, solve2, DAY};

    fn sample_input(qualifier: Option<&str>) -> String {
        get_input(DAY, true, qualifier)
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::I32(7), solve1(&sample_input(None)));
        assert_eq!(Solution::I32(5), solve1(&sample_input("2".into())));
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(19), solve2(&sample_input(None)));
        assert_eq!(Solution::I32(23), solve2(&sample_input("2".into())));
    }
}
