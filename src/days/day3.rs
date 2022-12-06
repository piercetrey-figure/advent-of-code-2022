use std::collections::HashSet;

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

pub fn solve() -> SolutionPair {
    let input = get_input(3, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let val = input
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|line| {
            let (left, right) = split_sack(line);
            let common = find_common_item(&left, &right).unwrap();
            item_priority(&common)
        })
        .sum();
    Solution::I32(val)
}

fn solve2(input: &str) -> Solution {
    let val = groups_of_3(input)
        .iter()
        .map(|(one, two, three)| {
            let common1and2 = find_common_items(one, two);
            let set3: HashSet<_> = three.split("").into_iter().map(|i| i.to_string()).collect();
            let intersection = common1and2.intersection(&set3).collect::<Vec<_>>();
            let common = intersection.first().unwrap();
            item_priority(common)
        })
        .sum();
    Solution::I32(val)
}

static LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn item_priority(item: &str) -> i32 {
    LETTERS.find(item).unwrap() as i32 + 1
}

fn split_sack(sack: &str) -> (String, String) {
    let items = sack.split("").into_iter().collect::<Vec<_>>();
    (
        items[0..items.len() / 2].join(""),
        items[items.len() / 2..items.len()].join(""),
    )
}

fn find_common_item(left: &str, right: &str) -> Option<String> {
    let left = left
        .split("")
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect::<HashSet<_>>();
    let right = right
        .split("")
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect::<HashSet<_>>();
    let intersection = left.intersection(&right);
    intersection
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|i| i.to_string())
}

fn find_common_items(left: &str, right: &str) -> HashSet<String> {
    let left = left
        .split("")
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect::<HashSet<_>>();
    let right = right
        .split("")
        .into_iter()
        .filter(|i| !i.is_empty())
        .collect::<HashSet<_>>();
    let intersection = left.intersection(&right);
    intersection.into_iter().map(|i| i.to_string()).collect()
}

fn groups_of_3(lines: &str) -> Vec<(String, String, String)> {
    lines
        .split("\n")
        .into_iter()
        .filter(|i| !i.is_empty())
        .array_chunks::<3>()
        .map(|chunk| {
            (
                chunk[0].to_string(),
                chunk[1].to_string(),
                chunk[2].to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{input::get_input, solution::Solution};

    use super::{find_common_item, groups_of_3, item_priority, solve1, solve2, split_sack};

    fn sample_input() -> String {
        get_input(3, true, None)
    }

    #[test]
    fn priority_test() {
        assert_eq!(1, item_priority("a"));
        assert_eq!(26, item_priority("z"));
        assert_eq!(27, item_priority("A"));
        assert_eq!(52, item_priority("Z"));
    }

    #[test]
    fn split_test() {
        assert_eq!(("123".to_string(), "456".to_string()), split_sack("123456"));
    }

    #[test]
    fn common_test() {
        assert_eq!("a", find_common_item("1a43", "bdaf").unwrap());
    }

    #[test]
    fn sample_1() {
        assert_eq!(Solution::I32(157), solve1(&sample_input()))
    }

    #[test]
    fn grouping_test() {
        assert_eq!(
            vec![
                ("1".to_string(), "2".to_string(), "3".to_string()),
                ("4".to_string(), "5".to_string(), "6".to_string()),
                ("7".to_string(), "8".to_string(), "9".to_string()),
            ],
            groups_of_3(
                "
1
2
3
4
5
6
7
8
9
            "
            )
        )
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::I32(70), solve2(&sample_input()));
    }
}
