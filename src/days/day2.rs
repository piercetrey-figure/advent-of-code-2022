use phf::phf_map;

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

pub fn solve() -> SolutionPair {
    let input = get_input(2, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let final_score = get_split_input(input)
        .iter()
        .map(|(them, you)| Type1Scorer::score_round(them, you))
        .sum();
    Solution::I32(final_score)
}

fn solve2(input: &str) -> Solution {
    let final_score = get_split_input(input)
        .iter()
        .map(|(them, you)| Type2Scorer::score_round(them, you))
        .sum();
    Solution::I32(final_score)
}

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_split_input(input: &str) -> Vec<(String, String)> {
    get_lines(input)
        .iter()
        .map(|line| {
            let pair: Vec<_> = line.split(" ").take(2).collect();
            (pair[0].into(), pair[1].into())
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

static PRETTY_SHAPE: phf::Map<&str, &str> = phf_map! {
    "A" => "Rock",
    "X" => "Rock",
    "B" => "Paper",
    "Y" => "Paper",
    "C" => "Scissors",
    "Z" => "Scissors",
};

static SHAPE_SCORES: phf::Map<&str, i32> = phf_map! {
    "Rock" => 1,
    "Paper" => 2,
    "Scissors" => 3,
};

static DEFEATS: phf::Map<&'static str, &'static str> = phf_map! {
    "Rock" => "Scissors",
    "Scissors" => "Paper",
    "Paper" => "Rock",
};

static REQUIRED_OUTCOME: phf::Map<&'static str, RoundOutcome> = phf_map! {
    "X" => RoundOutcome::Loss,
    "Y" => RoundOutcome::Draw,
    "Z" => RoundOutcome::Win,
};

fn round_outcome(them: &str, you: &str) -> RoundOutcome {
    if them == you {
        return RoundOutcome::Draw;
    }

    if *DEFEATS.get(you).unwrap() == them {
        return RoundOutcome::Win;
    }

    return RoundOutcome::Loss;
}

struct Type1Scorer {}
impl Type1Scorer {}
impl RoundScorer for Type1Scorer {
    fn score_round(them: &str, you: &str) -> i32 {
        let (them, you) = translate(them, you);
        let shape_score = *SHAPE_SCORES.get(&you).unwrap();
        let round_score = round_outcome(&them, &you) as i32;
        shape_score + round_score
    }
}

struct Type2Scorer {}
impl Type2Scorer {
    fn required_play(them: &str, outcome: RoundOutcome) -> String {
        match outcome {
            RoundOutcome::Loss => (*DEFEATS.get(them).unwrap()).into(),
            RoundOutcome::Draw => them.into(),
            RoundOutcome::Win => (*DEFEATS
                .into_iter()
                .find(|(_, b)| **b == them)
                .map(|(a, _)| a)
                .unwrap())
            .into(),
        }
    }
}
impl RoundScorer for Type2Scorer {
    fn score_round(them: &str, you: &str) -> i32 {
        let (them, _) = translate(them, you); // only translate them at this point
        let your_play = Self::required_play(&them, *REQUIRED_OUTCOME.get(you).unwrap());
        let shape_score = *SHAPE_SCORES.get(&your_play).unwrap();
        let round_score = round_outcome(&them, &your_play) as i32;
        shape_score + round_score
    }
}

trait RoundScorer {
    fn score_round(them: &str, you: &str) -> i32;
}

fn translate(them: &str, you: &str) -> (String, String) {
    (
        (*PRETTY_SHAPE.get(them).unwrap_or(&them)).into(),
        (*PRETTY_SHAPE.get(you).unwrap_or(&you)).into(),
    )
}

#[cfg(test)]
mod test {
    use crate::{
        days::day2::{RoundOutcome, RoundScorer, Type1Scorer, Type2Scorer},
        solution::Solution,
    };

    use super::{get_input, round_outcome, solve1, solve2};

    fn sample_input() -> String {
        get_input(2, true, None)
    }

    #[test]
    fn round_outcome_test() {
        // tie
        assert_eq!(RoundOutcome::Draw, round_outcome("Rock", "Rock"));
        assert_eq!(RoundOutcome::Draw, round_outcome("Paper", "Paper"));
        assert_eq!(RoundOutcome::Draw, round_outcome("Scissors", "Scissors"));
        // they win
        assert_eq!(RoundOutcome::Loss, round_outcome("Rock", "Scissors"));
        assert_eq!(RoundOutcome::Loss, round_outcome("Paper", "Rock"));
        assert_eq!(RoundOutcome::Loss, round_outcome("Scissors", "Paper"));
        // we win
        assert_eq!(RoundOutcome::Win, round_outcome("Scissors", "Rock"));
        assert_eq!(RoundOutcome::Win, round_outcome("Rock", "Paper"));
        assert_eq!(RoundOutcome::Win, round_outcome("Paper", "Scissors"));
    }

    #[test]
    fn round_score_test() {
        assert_eq!(8, Type1Scorer::score_round("A", "Y"));
        assert_eq!(1, Type1Scorer::score_round("B", "X"));
        assert_eq!(6, Type1Scorer::score_round("Scissors", "Scissors"));
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::I32(15), solve1(&sample_input()));
    }

    #[test]
    fn round_score_test_2() {
        assert_eq!(4, Type2Scorer::score_round("A", "Y"));
        assert_eq!(1, Type2Scorer::score_round("B", "X"));
        assert_eq!(7, Type2Scorer::score_round("Scissors", "Z"));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::I32(12), solve2(&sample_input()));
    }
}
