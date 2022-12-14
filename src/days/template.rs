use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = <day>;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    Solution::I32(0)
}

fn solve2(input: &str) -> Solution {
    Solution::I32(0)
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
        assert_eq!(Solution::I32(0), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(0), solve2(&sample_input()));
    }
}