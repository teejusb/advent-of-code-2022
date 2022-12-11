use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_cycle(cycle: i32, x: i32) -> i32 {
  if (cycle - 20) % 40 == 0 {
    return x * cycle;
  }
  return 0;
}

fn draw(cycle: i32, x: i32, screen: &mut Vec<Vec<char>>) {
  let s = max(0, x-1) as usize;
  let e = min(39, x+1) as usize;
  let i = ((cycle - 1) / 40) as usize;
  let j = ((cycle - 1) % 40) as usize;

  if s <= j && j <= e {
    screen[i][j] = '#';
  } else {
    screen[i][j] = '.';
  }
}

pub fn solve() {
  let file = File::open("src/days/day10/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut x: i32 = 1;
  let mut cycle: i32 = 1;

  let mut part1: i32 = 0;
  let mut screen: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    if s == "noop" {
      draw(cycle, x, &mut screen);
      cycle += 1;
      part1 += check_cycle(cycle, x);
    } else {
      let inc: i32 = s.split(" ").last().unwrap().parse().unwrap();
      
      draw(cycle, x, &mut screen);
      cycle += 1;
      part1 += check_cycle(cycle, x);

      draw(cycle, x, &mut screen);
      x += inc;
      cycle += 1;
      part1 += check_cycle(cycle, x);
    }
  }
  println!("{}", part1);

  for r in screen.iter() {
    for c in r.iter() {
      print!("{}", c);
    }
    println!("");
  }
}
