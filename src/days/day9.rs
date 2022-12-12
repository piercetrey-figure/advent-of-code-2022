use std::{
    collections::{HashMap, HashSet, VecDeque},
    num,
};

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
    let mut rope = Rope::init(input, 2);
    rope.advance_all();
    Solution::I32(rope.tails_visited[&1].len() as i32)
}

fn solve2(input: &str) -> Solution {
    let mut rope = Rope::init(input, 10);
    rope.advance_all();
    Solution::I32(rope.tails_visited[&9].len() as i32)
}

struct Rope {
    tails_visited: HashMap<i32, HashSet<(i32, i32)>>,
    positions: Vec<(i32, i32)>,
    moves: VecDeque<(String, i32)>,
}

impl Rope {
    fn init(input: &str, num_knots: i32) -> Self {
        let moves = input
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let elements: Vec<_> = line.split(" ").collect();
                (elements[0].to_string(), elements[1].parse().unwrap())
            })
            .collect();
        let mut tails_visited = HashMap::default();
        let positions = (0..num_knots)
            .map(|i| {
                tails_visited.insert(i, HashSet::from([(0, 0)]));
                (0, 0)
            })
            .collect();
        Self {
            tails_visited,
            positions,
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
                "U" => self.positions[0] = (self.positions[0].0, self.positions[0].1 + 1),
                "D" => self.positions[0] = (self.positions[0].0, self.positions[0].1 - 1),
                "L" => self.positions[0] = (self.positions[0].0 - 1, self.positions[0].1),
                "R" => self.positions[0] = (self.positions[0].0 + 1, self.positions[0].1),
                _ => panic!("unknown direction {}", dir),
            }
            self.advance_tails()
        }
    }

    fn advance_tails(&mut self) {
        for i in 1..self.positions.len() {
            if Self::distance(self.positions[i], self.positions[i - 1]) > 2f32.sqrt() {
                let dx = self.positions[i - 1].0 - self.positions[i].0;
                let dy = self.positions[i - 1].1 - self.positions[i].1;
                self.positions[i] = (
                    self.positions[i].0 + (dx.signum() * 1),
                    self.positions[i].1 + (dy.signum() * 1),
                );
                self.tails_visited.entry(i as i32).and_modify(|e| {
                    e.insert(self.positions[i]);
                });
            }
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
