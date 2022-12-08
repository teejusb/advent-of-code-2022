use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct Dir {
  pub size: u32,
  pub subdirs: HashSet<String>,
  pub files: HashSet<(String, u32)>,
}

impl Dir {
  fn new() -> Dir {
    return Dir{size: 0, subdirs: HashSet::new(), files: HashSet::new()};
  }
}

fn make_key(dir: &PathBuf) -> String{
  return dir.to_str().unwrap().replace("\\", "/").to_string();
}

fn update_sizes(fs: &mut HashMap<String, Dir>) {
  fn update_sizes_recursive(
      fs: &HashMap<String, Dir>,
      cur_dir: &mut PathBuf,
      new_sizes: &mut HashMap<String, u32>) -> u32{
    let key = make_key(cur_dir);
    let dir = &fs[&key];
    let mut total_subdir_size = 0;

    for subdir in dir.subdirs.iter() {
      cur_dir.push(subdir);
      let subdir_size = update_sizes_recursive(fs, cur_dir, new_sizes);
      total_subdir_size += subdir_size;
      cur_dir.pop();
    }

    let total_size = dir.size + total_subdir_size;
    new_sizes.insert(key, total_size);
    return total_size;
  }

  let mut new_sizes: HashMap<String, u32> = HashMap::new();
  update_sizes_recursive(fs, &mut PathBuf::from("/"), &mut new_sizes);

  for (key, size) in new_sizes.iter() {
    fs.get_mut(key).unwrap().size = *size;
  }
}

fn part1(fs: &HashMap<String, Dir>) -> u32 {
  let mut ans = 0;
  for (_, dir) in fs.iter() {
    if dir.size <= 100000 {
      ans += dir.size;
    }
  }
  return ans;
}

fn part2(fs: &HashMap<String, Dir>) -> u32 {
  let available = 70000000 - fs["/"].size;
  let to_clear = 30000000 - available;
  let mut ans = std::u32::MAX;
  
  for (_, dir) in fs.iter() {
    if dir.size >= to_clear {
      ans = min(ans, dir.size);
    }
  }
  return ans;
}

pub fn solve() {
  let file = File::open("src/days/day07/in.txt").expect("Cannot open file");
  let reader = BufReader::new(file);
  
  let mut fs: HashMap<String, Dir> = HashMap::new();
  let mut cur_dir = PathBuf::new();
  for line in reader.lines() {
    let s = line.expect("Error reading the line");
    let parts: Vec<&str> = s.split(" ").collect();
    // Is a command
    if parts[0] == "$" {
      if parts[1] == "cd" {
        if parts[2] == ".." {
          cur_dir.pop();
        } else {
          cur_dir.push(parts[2]);
          // If this is the first time we're encountering this directory,
          // create a new vector for it.
          let key = make_key(&cur_dir);
          if !fs.contains_key(&key) {
            fs.insert(key, Dir::new());
          }
        }
      }
    // Is part of an ls listing.
    } else {
      let key = make_key(&cur_dir);
      let dir = fs.get_mut(&key).unwrap();
      // Is a directory
      if parts[0] == "dir" {
        if !dir.subdirs.contains(parts[1]) {
          dir.subdirs.insert(parts[1].to_string());
        }
      // Is a file
      } else {
        let size: u32 = parts[0].parse().unwrap();
        dir.files.insert((parts[1].to_string(), size));
        dir.size += size;
      }
    }
  }
  // Recompute total dir size including sub directories.
  update_sizes(&mut fs);

  println!("{}", part1(&fs));
  println!("{}", part2(&fs));
}