use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/day05/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut diagram_str: Vec<String> = Vec::new();
  let mut reading_diagram = true;
  let mut part1: Vec<Vec<char>> = Vec::new();
  let mut part2: Vec<Vec<char>> = Vec::new();
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    if s.is_empty() {
      // How many buckets do we have?
      let n = diagram_str.last().unwrap().trim().split(" ").last().unwrap()
                         .parse().unwrap();
      part1.resize(n, Vec::new());
      part2.resize(n, Vec::new());

      // Iterate backwards to build the stack.
      for i in (0..diagram_str.len()-1).rev() {
        for j in 1..diagram_str[i].len() {
          if diagram_str[i].chars().nth(j-1).unwrap() == '[' {
            part1[j / 4].push(diagram_str[i].chars().nth(j).unwrap());
            part2[j / 4].push(diagram_str[i].chars().nth(j).unwrap());
          }
        }
      }
      reading_diagram = false;
      continue;
    }

    if reading_diagram {
      diagram_str.push(s);
    } else {
      let parts: Vec<&str> = s.split(" ").collect();
      let n = parts[1].parse().unwrap();
      let from: usize = parts[3].parse().unwrap();
      let to: usize = parts[5].parse().unwrap();

      // Part 1
      for _ in 0..n {
        let val = part1[from-1].pop().unwrap();
        part1[to-1].push(val);
      }

      // Part 2
      {
        let sz = part2[from-1].len();
        let val = part2[from-1].split_off(sz-n);
        part2[to-1].extend(val.iter());
      }
    }
  }

  for d in part1.iter() {
    print!("{}", d.last().unwrap());
  }
  println!("");

  for d in part2.iter() {
    print!("{}", d.last().unwrap());
  }
  println!("");
}