use std::{collections::HashSet, fmt::Display};

use phf::{phf_map, Set};

use crate::{
    input::get_input,
    solution::{Solution, SolutionPair},
};

static DAY: i32 = 10;

pub fn solve() -> SolutionPair {
    let input = get_input(DAY, false, None);
    (solve1(&input), solve2(&input))
}

fn solve1(input: &str) -> Solution {
    let (mut crt, commands) = parse_input(input);
    let signal_strength = crt.iterate_commands(&commands);
    Solution::I32(signal_strength)
}

fn solve2(input: &str) -> Solution {
    let (mut crt, commands) = parse_input(input);
    let pixels = crt.iterate_commands_pixels(&commands);
    let screen = pixels.split("").enumerate().map(|(i, e)| {
        if i != 1 && i % 40 == 1 {
            format!("\n{}", e)
        } else {
            e.to_string()
        }
    }).collect::<Vec<_>>().join("");
    Solution::String(screen)
}

struct CRT {
    cycle: i32,
    current_command: Option<Command>,
    X: i32,
}

impl CRT {
    fn set_command(&mut self, cmd: Command) -> () {
        self.current_command = Some(cmd);
    }

    fn iterate(&mut self) -> () {
        self.cycle += 1;
        match self.current_command {
            Some(Command::AddX(cycles_remaining, value)) => {
                if cycles_remaining > 1 {
                    self.current_command = Some(Command::AddX(cycles_remaining - 1, value));
                } else {
                    self.current_command = None;
                    self.X += value;
                }
            },
            Some(Command::Noop(cycles_remaining)) => {
                if cycles_remaining > 1 {
                    self.current_command = Some(Command::Noop(cycles_remaining - 1));
                } else {
                    self.current_command = None;
                }
            }
            None => panic!("Cannot iterate without a current command"),
        }
    }

    fn iterate_commands(&mut self, commands: &[Command]) -> i32 {
        let mut signal_strength = 0;
        for command in commands {
            self.set_command(*command);
            while let Some(_) = self.current_command {
                self.iterate();

                if (self.cycle - 20) % 40 == 0 {
                    signal_strength += self.cycle * self.X;
                }
            }
        }
        signal_strength
    }

    fn iterate_commands_pixels(&mut self, commands: &[Command]) -> String {
        let mut pixels = "".into();
        for command in commands {
            self.set_command(*command);
            while let Some(_) = self.current_command {
                pixels = format!("{}{}", pixels, self.pixel());
                self.iterate();
            }
        }
        pixels
    }

    fn pixel(&self) -> char {
        if ((self.X - 1) % 40) == ((self.cycle - 1) % 40) || ((self.X) % 40) == ((self.cycle - 1) % 40) || ((self.X + 1) % 40) == ((self.cycle - 1) % 40) {
            '#'
        } else {
            '.'
        }
    }
}

#[derive(Copy, Clone)]
enum Command {
    AddX(usize, i32),
    Noop(usize)
}

fn parse_input(input: &str) -> (CRT, Vec<Command>) {
    let commands = input.split("\n").filter(|line| !line.trim().is_empty()).map(|line| {
        let elems: Vec<_> = line.split(" ").collect();
        match elems.first() {
            Some(&"addx") => Command::AddX(2, elems[1].parse().unwrap()),
            Some(&"noop") => Command::Noop(1),
            other => panic!("Unknown command {:#?}", other)
        }
    }).collect();
    (CRT {
        cycle: 1,
        current_command: None,
        X: 1,
    }, commands)
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
        assert_eq!(Solution::I32(13140), solve1(&sample_input()))
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::String("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".into()), solve2(&sample_input()));
    }
}