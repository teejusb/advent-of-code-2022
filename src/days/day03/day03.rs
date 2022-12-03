use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_value(c: char) -> u32 {
  let val = c as u32;
  // Uppercase
  if 65 <= val && val <= 90 {
    return val - 38;
  }

  // Lowercase
  if 97 <= val && val <= 122 {
    return val - 96;
  }
  return 0;
}

fn part1() {
  let file = File::open("src/days/day03/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut sum: u32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let n = s.len();
    let (first, second) = s.split_at(n/2);
    let chars1: HashSet<char> = first.chars().collect();
    let chars2: HashSet<char> = second.chars().collect();
    let intersection: HashSet<char> =
        chars1.intersection(&chars2).copied().collect();
    sum += char_value(*intersection.iter().next().unwrap());
  }
  println!("{}", sum);
}

fn part2() {
  let file = File::open("src/days/day03/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let group_size = 3;
  let mut sum: u32 = 0;
  let mut intersection: HashSet<char> = HashSet::new();
  let mut line_n: u32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    if line_n == 0 {
      intersection = s.chars().collect();
    } else {
      let chars: HashSet<char> = s.chars().collect();
      intersection = intersection.intersection(&chars).copied().collect();
    }

    line_n += 1;
    if line_n == group_size {
      sum += char_value(*intersection.iter().next().unwrap());
      intersection.clear();
      line_n = 0;
    }
  }
  println!("{}", sum);
}

pub fn solve() {
  part1();
  part2();
}