use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/day06/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    for part in [4, 14].iter() {
      for end in *part..s.len() {
        let ss = &s[(end-part)..end];
        let count: HashSet<char> = HashSet::from_iter(ss.chars());
        if count.len() == *part {
          println!("{}", end);
          break;
        }
      }
    }
  }
}