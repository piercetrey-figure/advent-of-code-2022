use std::{collections::HashMap, fmt::Debug};

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 7;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let fs = FS::build_from_input(input);
    let sum_dirs_gt_100k: usize = fs
        .dirs
        .iter()
        .map(|(name, _)| fs.dir_size(name))
        .filter(|size| size <= &100000)
        .sum();
    Solution::I32(sum_dirs_gt_100k as i32)
}

fn solve2(input: &str) -> Solution {
    const REQUIRED_SPACE: usize = 30000000;
    const TOTAL_SPACE: usize = 70000000;

    let fs = FS::build_from_input(input);

    let unused_space = TOTAL_SPACE - fs.dir_size("/");
    let to_free = REQUIRED_SPACE - unused_space;

    let mut delete_options: Vec<_> = fs
        .dirs
        .iter()
        .map(|(name, _)| fs.dir_size(name))
        .filter(|size| size > &to_free)
        .collect();

    delete_options.sort();

    Solution::I32(*delete_options.first().unwrap() as i32)
}

#[derive(Debug)]
struct FileInfo {
    pub name: String,
    pub size: usize,
}

#[derive(Debug)]
struct DirInfo {
    pub name: String,
}

#[derive(Debug)]
enum DirEntry {
    Dir(DirInfo),
    File(FileInfo),
}

struct FS {
    pub dirs: HashMap<String, Vec<DirEntry>>,
}

impl FS {
    fn build_from_input(input: &str) -> FS {
        let lines: Vec<_> = input.split("\n").collect();
        let mut cwd = "".to_string();
        let mut dirs: HashMap<String, Vec<DirEntry>> = HashMap::new();
        for line in lines {
            if line.starts_with("$") {
                let command: Vec<_> = line[2..line.len()].split(" ").collect();
                match command[0] {
                    "cd" => match command[1] {
                        ".." => {
                            let elements = cwd.split("/").collect::<Vec<_>>();
                            cwd = elements[0..elements.len() - 1].join("/");
                        }
                        "/" => cwd = "/".to_string(),
                        _ => {
                            if cwd != "/" {
                                cwd = format!("{}/{}", cwd, command[1]);
                            } else {
                                cwd = format!("/{}", command[1]);
                            }
                        }
                    },
                    "ls" => {}
                    _ => panic!("Unknown command {}", line),
                }
            } else {
                let dir_info: Vec<_> = line.split(" ").collect();
                let name = dir_info[1];
                let dir_info = if dir_info[0] == "dir" {
                    DirEntry::Dir(DirInfo {
                        name: name.to_string(),
                    })
                } else {
                    DirEntry::File(FileInfo {
                        name: name.to_string(),
                        size: dir_info[0].parse().unwrap(),
                    })
                };
                dirs.entry(cwd.clone()).or_default().push(dir_info);
            }
        }
        FS { dirs }
    }

    fn dir_size(&self, dir: &str) -> usize {
        self.dirs
            .get(dir)
            .map(|files| {
                files
                    .iter()
                    .map(|f| match f {
                        DirEntry::Dir(DirInfo { name }) => {
                            if dir == "/" {
                                self.dir_size(format!("/{}", name).as_str())
                            } else {
                                self.dir_size(format!("{}/{}", dir, name).as_str())
                            }
                        }
                        DirEntry::File(FileInfo { size, .. }) => *size,
                    })
                    .sum()
            })
            .unwrap_or(0)
    }
}

impl Debug for FS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FS").field("dirs", &self.dirs).finish()
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
        assert_eq!(Solution::I32(95437), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(24933642), solve2(&sample_input()));
    }
}
