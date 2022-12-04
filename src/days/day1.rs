use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

pub fn solve() -> SolutionPair {
    let input = get_input(1, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    Solution::I32(*get_sums(input).iter().max().unwrap())
}

fn solve2(input: &str) -> Solution {
    let mut sums = get_sums(input);
    sums.sort();
    let top3 = sums
        .len()
        .checked_sub(3)
        .map(|i| sums[i..i + 3].iter().sum())
        .unwrap();
    Solution::I32(top3)
}

fn get_sums(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .into_iter()
        .map(|lines| {
            lines
                .split("\n")
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|calories| -> i32 { calories.parse().unwrap() })
                .sum()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{solve1, solve2};

    fn sample_input() -> String {
        get_input(1, true, None)
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::I32(24000), solve1(&sample_input()));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::I32(45000), solve2(&sample_input()));
    }
}
