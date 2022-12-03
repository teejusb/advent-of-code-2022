use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/template/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut sum: i32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
  }
  println!("{}", sum);
}