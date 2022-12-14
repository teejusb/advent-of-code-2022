mod days;

use std::env;
use days::{day01, day02, day03, day04, day05, day06, day07, day08, day09,
           day10, day11, day12, day13};

fn get_day_solver(day: i32) -> fn() {
  match day {
    1 => day01::day01::solve,
    2 => day02::day02::solve,
    3 => day03::day03::solve,
    4 => day04::day04::solve,
    5 => day05::day05::solve,
    6 => day06::day06::solve,
    7 => day07::day07::solve,
    8 => day08::day08::solve,
    9 => day09::day09::solve,
    10 => day10::day10::solve,
    11 => day11::day11::solve,
    12 => day12::day12::solve,
    13 => day13::day13::solve,
    _ => unimplemented!(),
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
      panic!("Please provide the day to run as a command-line argument.");
  }

  let day: i32 = args[1].parse().unwrap();

  let func: fn() = get_day_solver(day);
  func();
}
