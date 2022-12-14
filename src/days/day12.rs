use std::collections::{HashMap, HashSet};

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 12;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let map = HeightMap::from_input(input);
    let min_distance = map.seek_end(map.start).unwrap();
    Solution::U64(min_distance)
}

fn solve2(input: &str) -> Solution {
    let map = HeightMap::from_input(input);
    let min_distance = map.seek_shortest().unwrap();
    Solution::U64(min_distance)
}

#[derive(Clone)]
struct HeightMap {
    elevations: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

static CARDINAL_DIRECTIONS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];

impl HeightMap {
    fn from_input(input: &str) -> Self {
        let elevations: Vec<Vec<char>> = input
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| line.chars().collect())
            .collect();

        Self {
            start: Self::find_index(elevations.clone(), 'S'),
            end: Self::find_index(elevations.clone(), 'E'),
            elevations,
        }
    }

    fn find_index(elevations: Vec<Vec<char>>, c: char) -> (usize, usize) {
        for y in 0..elevations.len() {
            for x in 0..elevations[y].len() {
                if elevations[y][x] == c {
                    return (x, y);
                }
            }
        }
        panic!("Char {} not found", c);
    }

    fn seek_end(&self, start: (usize, usize)) -> Option<u64> {
        let mut curr = start;
        let mut depth = 0;
        let mut shortest_distance: HashMap<(usize, usize), u64> = self
            .elevations
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| (x, y)))
            .map(|n| (n, u64::MAX))
            .collect();
        shortest_distance.insert(curr, depth);
        let mut visited: HashSet<(usize, usize)> = HashSet::default();
        while !visited.contains(&self.end) {
            let neighbors = self.neighbors(&curr);

            for neighbor in &neighbors {
                let neighbor_dist = shortest_distance.get_mut(neighbor).unwrap();
                if *neighbor_dist > depth + 1 {
                    *neighbor_dist = depth + 1;
                }
            }

            visited.insert(curr);

            if let Some(next) = shortest_distance
                .iter()
                .filter(|(n, _)| !visited.contains(n))
                .min_by_key(|(_, v)| **v)
            {
                curr = *next.0;
                depth = *next.1;
            } else {
                break;
            }
        }
        Some(*shortest_distance.get(&self.end).unwrap())
    }

    fn seek_shortest(&self) -> Option<u64> {
        let mut curr = self.end;
        let mut depth = 0;
        let mut shortest_distance: HashMap<(usize, usize), u64> = self
            .elevations
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| (x, y)))
            .map(|n| (n, u64::MAX))
            .collect();
        shortest_distance.insert(curr, depth);
        let mut visited: HashSet<(usize, usize)> = HashSet::default();
        loop {
            let neighbors = self.neighbors2(&curr);

            for neighbor in &neighbors {
                let neighbor_dist = shortest_distance.get_mut(neighbor).unwrap();
                if *neighbor_dist > depth + 1 {
                    *neighbor_dist = depth + 1;
                }
            }

            visited.insert(curr);

            if let Some(next) = shortest_distance
                .iter()
                .filter(|(n, _)| !visited.contains(n))
                .min_by_key(|(_, v)| **v)
            {
                if *next.1 == u64::MAX {
                    // no reachable nodes remain
                    break;
                }

                curr = *next.0;
                depth = *next.1;
            } else {
                break;
            }
        }
        Some(
            *shortest_distance
                .iter()
                .filter(|(n, _)| self.elevation(n) == 'a')
                .min_by_key(|(_, d)| *d)
                .map(|(_, d)| d)
                .unwrap(),
        )
    }

    fn neighbors(&self, curr: &(usize, usize)) -> Vec<(usize, usize)> {
        CARDINAL_DIRECTIONS
            .iter()
            .map(|(dx, dy)| {
                let x = curr.0 as i32 + *dx;
                let y = curr.1 as i32 + *dy;
                if x >= self.elevations[0].len() as i32 // out of bounds
                        || x < 0
                        || y >= self.elevations.len() as i32
                        || y < 0
                {
                    None
                } else {
                    Some((x as usize, y as usize))
                }
            })
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .filter(|n| Self::delta(self.elevation(curr), self.elevation(n)) < 2)
            .collect()
    }

    fn neighbors2(&self, curr: &(usize, usize)) -> Vec<(usize, usize)> {
        CARDINAL_DIRECTIONS
            .iter()
            .map(|(dx, dy)| {
                let x = curr.0 as i32 + *dx;
                let y = curr.1 as i32 + *dy;
                if x >= self.elevations[0].len() as i32 // out of bounds
                        || x < 0
                        || y >= self.elevations.len() as i32
                        || y < 0
                {
                    None
                } else {
                    Some((x as usize, y as usize))
                }
            })
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .filter(|n| Self::delta(self.elevation(curr), self.elevation(n)) > -2)
            .collect()
    }

    fn elevation(&self, (x, y): &(usize, usize)) -> char {
        self.elevations[*y][*x]
    }

    fn delta(c1: char, c2: char) -> i32 {
        let substitutions = HashMap::from([('S', 'a'), ('E', 'z')]);
        *substitutions.get(&c2).unwrap_or(&c2) as i32
            - *substitutions.get(&c1).unwrap_or(&c1) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{solve1, solve2, HeightMap, DAY};

    fn sample_input() -> String {
        get_input(DAY, true, None)
    }

    #[test]
    fn test_compare() {
        assert_eq!(2, HeightMap::delta('a', 'c'));
        assert_eq!(2, HeightMap::delta('S', 'c'));
        assert_eq!(25, HeightMap::delta('S', 'E'));
        assert_eq!(0, HeightMap::delta('S', 'a'));
        assert_eq!(0, HeightMap::delta('E', 'z'));
        assert_eq!(4, HeightMap::delta('v', 'E'));
        assert_eq!(-1, HeightMap::delta('z', 'y'));
        assert_eq!(-2, HeightMap::delta('z', 'x'));
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::U64(31), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::U64(29), solve2(&sample_input()));
    }
}
