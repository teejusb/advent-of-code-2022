use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/day04/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut part1: u32 = 0;
  let mut part2: u32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let mut pairs: Vec<Vec<u32>> = s.split(",").map(|parts| {
        parts.split("-").map(|vals| {
            vals.parse().unwrap()
        }).collect()
    }).collect();

    pairs.sort_by(|a, b| {
      if a[0] == b[0] {
        return b[1].cmp(&a[1]);
      } else {
        return a.cmp(b);
      }
    });

    if pairs[0][1] >= pairs[1][1] {
      part1 += 1;
    }

    if pairs[0][1] >= pairs[1][0] {
      part2 += 1;
    }
  }
  println!("{}", part1);
  println!("{}", part2);
}