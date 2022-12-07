#![feature(iter_array_chunks)]
#![feature(array_windows)]

use std::{env, time::Instant};

mod days;
mod input;
mod solution;

use solution::SolutionPair;

use crate::days::{day1, day2, day3, day4, day5, day6, day7};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide the day to run as a command-line argument");
    }

    let day_arg: u32 = args[1].parse().unwrap();

    let runner = get_day(day_arg);

    let time = Instant::now();
    let (r1, r2) = runner();
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("==== Day {:02} ====", day_arg);
    println!(" . Part 1: {}", r1);
    println!(" . Part 2: {}", r2);
    println!(" . Elapased: {:.4} ms", elapsed_ms);
}

fn get_day(day: u32) -> fn() -> SolutionPair {
    match day {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        5 => day5::solve,
        6 => day6::solve,
        7 => day7::solve,
        _ => unimplemented!("day {} is unimplemented", day),
    }
}
