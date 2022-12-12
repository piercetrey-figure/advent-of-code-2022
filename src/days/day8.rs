use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 8;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let grid = Grid::new(input);
    Solution::I32(grid.count_visible())
}

fn solve2(input: &str) -> Solution {
    let grid = Grid::new(input);
    Solution::I32(grid.highest_senic_score())
}

struct Grid {
    trees: Vec<Vec<i32>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let trees = input
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                line.split("")
                    .filter(|i| !i.trim().is_empty())
                    .map(|t| t.parse().unwrap())
                    .collect()
            })
            .collect();
        Self { trees }
    }

    fn count_visible(&self) -> i32 {
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
            .filter(|(x, y)| self.is_visible(*x as i32, *y as i32))
            .collect::<Vec<_>>()
            .len() as i32
    }

    fn highest_senic_score(&self) -> i32 {
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
            .map(|(x, y)| self.senic_score(x as i32, y as i32))
            .max()
            .unwrap()
    }

    fn is_visible(&self, x: i32, y: i32) -> bool {
        vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .any(|(dx, dy)| {
                if dx != &0 {
                    let mut i = x + dx;
                    while i < self.trees.first().unwrap().len() as i32 && i >= 0 {
                        if self.trees[y as usize][i as usize] >= self.trees[y as usize][x as usize]
                        {
                            return false;
                        }
                        i += dx;
                    }
                }
                if dy != &0 {
                    let mut j = y + dy;
                    while j < self.trees.len() as i32 && j >= 0 {
                        if self.trees[j as usize][x as usize] >= self.trees[y as usize][x as usize]
                        {
                            return false;
                        }
                        j += dy;
                    }
                }
                true
            })
    }

    fn senic_score(&self, x: i32, y: i32) -> i32 {
        vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(|(dx, dy)| {
                let mut num_visible = 0;
                if dx != &0 {
                    let mut i = x + dx;
                    while i < self.trees.first().unwrap().len() as i32 && i >= 0 {
                        num_visible += 1;
                        if self.trees[y as usize][i as usize] >= self.trees[y as usize][x as usize]
                        {
                            break;
                        }
                        i += dx;
                    }
                }
                if dy != &0 {
                    let mut j = y + dy;
                    while j < self.trees.len() as i32 && j >= 0 {
                        num_visible += 1;
                        if self.trees[j as usize][x as usize] >= self.trees[y as usize][x as usize]
                        {
                            break;
                        }
                        j += dy;
                    }
                }
                num_visible
            })
            .product()
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
        assert_eq!(Solution::I32(21), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(8), solve2(&sample_input()));
    }
}
