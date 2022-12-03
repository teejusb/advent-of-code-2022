mod days;

use std::env;
use days::{day01, day02};

fn get_day_solver(day: i32) -> fn() {
  match day {
    1 => day01::day01::solve,
    2 => day02::day02::solve,
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