use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
  // <= 2 works since we're in 2-D and only looking for 1 unit away.
  return (a.0 - b.0).pow(2) + (a.1-b.1).pow(2) <= 2
}

fn ans(steps: &Vec<(String, i32)>, knots: usize) -> usize{
  let dir_map: HashMap<String, (i32, i32)> = HashMap::from([
    ("R".to_string(), (1, 0)),
    ("L".to_string(), (-1, 0)),
    ("D".to_string(), (0, -1)),
    ("U".to_string(), (0, 1)),
  ]);
  
  let mut rope = vec![(0,0); knots];
  let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

  for (dir, mag) in steps.iter() {
    let (x, y) = dir_map[dir];
    for _ in 0..*mag {
      rope[0].0 += x;
      rope[0].1 += y;
      for i in 1..rope.len() {
        if !adjacent(rope[i], rope[i-1]) {
          rope[i].0 += max(-1, min(1, rope[i-1].0 - rope[i].0));
          rope[i].1 += max(-1, min(1, rope[i-1].1 - rope[i].1));
        }
      }
      visited.insert(*rope.last().unwrap());
    }
  }
  return visited.len();
}

pub fn solve() {
  let file = File::open("src/days/day09/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut steps: Vec<(String, i32)> = Vec::new();
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let parts: Vec<&str> = s.split(" ").collect();
    let dir: &str = parts[0];
    let mag: i32 = parts[1].parse().unwrap();
    steps.push((dir.to_string(), mag));
  }

  println!("{}", ans(&steps, 2));
  println!("{}", ans(&steps, 10));
}