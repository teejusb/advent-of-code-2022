use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
  let file = File::open("src/days/day08/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut sum: i32 = 0;
  let mut score: i32 = 0;
  let mut grid: Vec<Vec<u32>> = Vec::new();
  
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let nums: Vec<u32> =
        s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    grid.push(nums);
  }
  let height = grid.len() as i32;
  let width = grid[0].len() as i32;

  sum += width * 2 + height * 2 - 4;

  let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

  for i in 1..height-1 {
    for j in 1..width-1 {
      let tree = grid[i as usize][j as usize];
      
      // Is this tree visible?
      let mut visible = false;
      let mut cur_score = 1;

      for dir in dirs.iter() {
        let mut i2 = i as i32 + dir.0;
        let mut j2 = j as i32 + dir.1;

        // Is this tree visible from this specific direction?
        let mut visibile_from_dir = true;
        let mut dir_score = 0;

        while 0 <= i2 && i2 < width && 0 <= j2 && j2 < height {
          let other = grid[i2 as usize][j2 as usize];
          dir_score += 1;

          if tree <= other {
            visibile_from_dir = false;
            break;
          }
          
          i2 += dir.0;
          j2 += dir.1;
        }
        visible |= visibile_from_dir;
        cur_score *= dir_score;
      }
      score = std::cmp::max(score, cur_score);
      if visible {
        sum += 1;
      }
    }
  }

  println!("{}", sum);
  println!("{}", score);
}