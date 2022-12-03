use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/day01/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut sum: i32 = 0;
  let mut heap = BinaryHeap::new();
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    if !s.is_empty() {
      sum += s.trim().parse::<i32>().unwrap();
    } else {
      heap.push(sum);
      sum = 0;
    }
  }
  heap.push(sum);

  sum = 0;
  for _ in 1..4 {
    if !heap.is_empty() {
      sum += heap.pop().unwrap();
    }
  }
  println!("{}", sum);
}