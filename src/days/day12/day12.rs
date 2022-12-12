use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bfs(e: (usize, usize), s: char, m: &Vec<Vec<char>>) -> i32 {
  let height = m.len() as i32;
  let width = m[0].len() as i32;

  let mut visited = HashSet::from([e]);
  let mut q = VecDeque::from([(e, 0)]);

  let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
  while !q.is_empty() {
    let (u, d) = q.pop_front().unwrap();
    let c = m[u.0][u.1];
    if c == s {
      return d;
    }

    for (dx, dy) in dirs.iter() {
      let x = u.0 as i32 + dx;
      let y = u.1 as i32 + dy;

      if 0 <= x && x < height && 0 <= y && y < width {
        let xu = x as usize;
        let yu = y as usize;
        let v = (xu, yu);

        let c_val = c as u32;
        let mut o = m[xu][yu];
        if o == 'S' {
          o = 'a';
        }
        let o_val = o as u32;

        if o_val + 1 >= c_val {
          if !visited.contains(&v) {
            visited.insert(v);
            q.push_back((v, d + 1));
          }
        }
      }
    }
  }
  return -1;
}

pub fn solve() {
  let file = File::open("src/days/day12/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);

  let mut elevation: Vec<Vec<char>> = Vec::new();
  let mut s = (0, 0);
  let mut e = (0, 0);
 
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    elevation.push(s.trim().chars().collect());
  }
  
  for i in 0..elevation.len() {
    for j in 0..elevation[i].len() {
      let c = elevation[i][j];
      if c == 'S'{
        s = (i, j);
      } else if c == 'E' {
        e = (i, j);
      }
    }
  }

  elevation[e.0][e.1] = 'z';
  println!("{}", bfs(e, 'S', &elevation));
  elevation[s.0][s.1] = 'a';
  println!("{}", bfs(e, 'a', &elevation));
}