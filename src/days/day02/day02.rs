use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
  let file = File::open("src/days/day02/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let map = HashMap::from([
    ("A", "Rock"),
    ("B", "Paper"),
    ("C", "Scissors"),
    ("X", "Rock"),
    ("Y", "Paper"),
    ("Z", "Scissors"),
  ]);
  let winning = HashMap::from([
    ("Rock", "Scissors"),
    ("Paper", "Rock"),
    ("Scissors", "Paper"),
  ]);
  let score = HashMap::from([
    ("Rock", 1),
    ("Paper", 2),
    ("Scissors", 3),
  ]);
  let mut sum: i32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let vec: Vec<&str> = s.split(" ").collect();
    let you = map[vec[0]];
    let me = map[vec[1]];

    if you == me {
      sum += 3 + score[me];
    } else if winning[you] != me {
      sum += 6 + score[me];
    } else {
      sum += score[me];
    }
  }
  println!("{}", sum);
}

fn part2() {
  let file = File::open("src/days/day02/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let map = HashMap::from([
    ("A", "Rock"),
    ("B", "Paper"),
    ("C", "Scissors"),
    ("X", "Rock"),
    ("Y", "Paper"),
    ("Z", "Scissors"),
  ]);
  let winning = HashMap::from([
    ("Rock", "Scissors"),
    ("Paper", "Rock"),
    ("Scissors", "Paper"),
  ]);
  let losing = HashMap::from([
    ("Scissors", "Rock"),
    ("Rock", "Paper"),
    ("Paper", "Scissors"),
  ]);
  let score = HashMap::from([
    ("Rock", 1),
    ("Paper", 2),
    ("Scissors", 3),
  ]);
  let mut sum: i32 = 0;
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let vec: Vec<&str> = s.split(" ").collect();
    let you = map[vec[0]];
    let outcome = vec[1];

    // Loss.
    if outcome == "X" {
      sum += score[winning[you]];
    // Draw.
    } else if outcome == "Y" {
      sum += 3 + score[you];
    // Win
    } else {
      sum += 6 + score[losing[you]];
    }
  }
  println!("{}", sum);
}

pub fn solve() {
  part1();
  part2();
}