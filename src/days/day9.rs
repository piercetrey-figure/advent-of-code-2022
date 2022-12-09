use std::collections::{HashSet, VecDeque};

use phf::{phf_map, Set};

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 9;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let mut rope = Rope::init(input);
    rope.advance_all();
    Solution::I32(rope.tail_visited.len() as i32)
}

fn solve2(input: &str) -> Solution {
    Solution::I32(0)
}

struct Rope {
    tail_visited: HashSet<(i32, i32)>,
    head_position: (i32, i32),
    tail_position: (i32, i32),
    moves: VecDeque<(String, i32)>,
}

impl Rope {
    fn init(input: &str) -> Self {
        let moves = input
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let elements: Vec<_> = line.split(" ").collect();
                (elements[0].to_string(), elements[1].parse().unwrap())
            })
            .collect();
        Self {
            tail_visited: HashSet::from([(0, 0)]),
            head_position: (0, 0),
            tail_position: (0, 0),
            moves,
        }
    }

    fn advance_all(&mut self) {
        while !self.moves.is_empty() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        let (dir, amt) = self.moves.pop_front().unwrap();
        for i in (0..amt) {
            match dir.as_str() {
                "U" => self.head_position = (self.head_position.0, self.head_position.1 + 1),
                "D" => self.head_position = (self.head_position.0, self.head_position.1 - 1),
                "L" => self.head_position = (self.head_position.0 - 1, self.head_position.1),
                "R" => self.head_position = (self.head_position.0 + 1, self.head_position.1),
                _ => panic!("unknown direction {}", dir),
            }
            self.advance_tail()
        }
    }

    fn advance_tail(&mut self) {
        if Self::distance(self.tail_position, self.head_position) > 2f32.sqrt() {
            let dx = self.head_position.0 - self.tail_position.0;
            let dy = self.head_position.1 - self.tail_position.1;
            self.tail_position = (
                self.tail_position.0 + (dx.signum() * 1),
                self.tail_position.1 + (dy.signum() * 1),
            );
            self.tail_visited.insert(self.tail_position);
        }
    }

    fn distance(a: (i32, i32), b: (i32, i32)) -> f32 {
        (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt()
    }
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
        assert_eq!(Solution::I32(13), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(1), solve2(&sample_input()));
    }

    #[test]
    fn sample_2_v2() {
        assert_eq!(Solution::I32(36), solve2(&get_input(DAY, true, Some("2"))));
    }
}
