use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
  List(Vec<Packet>),
  Num(i32),
}

impl Packet {
  fn parse(s: &str, mut i: usize) -> (Self, usize) {
    let chars: Vec<char> = s.chars().collect();
    if chars[i] == '[' {
      let mut res: Vec<Packet> = Vec::new();
      while i < chars.len() {
        i += 1;

        if chars[i] == '[' {
          // Nested list. Recurse.
          let (packet, len) = Packet::parse(s, i);
          i = len;
          res.push(packet);
        } else {
          // Start of a number.
          let start = i;
          while chars[i] != ',' && chars[i] != ']' {
            i += 1;
          }

          if start != i {
            res.push(Packet::Num(
              chars[start..i].to_vec().iter().collect::<String>()
                            .parse().unwrap()));
          }
        }
        // We parsed the last element in this list.
        // Return the newly finished packet or sub-packet.
        if i < chars.len() && chars[i] == ']' {
          // We do i + 1 to move the index past the closing brace of the current
          // packet.
          return (Packet::List(res), i + 1);
        }
      }
    }
    return (Packet::Num(0), 0);
  }
}

fn cmp(l: &Packet, r: &Packet) -> Ordering {
  match (l, r) {
    (Packet::Num(left), Packet::Num(right)) => {
      if left < right {
        return Ordering::Less;
      } else if left == right {
        return Ordering::Equal;
      } else {
        return Ordering::Greater;
      }
    }
    (Packet::Num(left), Packet::List(right)) => {
      return cmp(&Packet::List(vec![Packet::Num(*left)]),
                  &Packet::List(right.to_vec()));
    }
    (Packet::List(left), Packet::Num(right)) => {
      return cmp(&Packet::List(left.to_vec()),
                  &Packet::List(vec![Packet::Num(*right)]));
    }
    (Packet::List(left), Packet::List(right)) => {
      for (i, p1) in left.iter().enumerate() {
        // We've run out of elements in the right list.
        if i >= right.len() {
          return Ordering::Greater;
        }
        let p2 = &right[i];
        let ans = cmp(p1, p2);
        if ans != Ordering::Equal {
          return ans;
        }
      }
      if left.len() == right.len() {
        let n = left.len();
        if n == 0 || left[n-1] == right[n-1] {
          return Ordering::Equal;
        }
      }
      return Ordering::Less;
    }
  };
}

pub fn solve() {
  let contents = fs::read_to_string("src/days/day13/in.txt")
                    .expect("Cannot open file").replace("\r\n", "\n");
  
  let mut sum: usize = 0;
  let mut all_packets: Vec<Packet> = Vec::new();
  for (i, chunk) in contents.split("\n\n").enumerate() {
    let packet_str: Vec<&str> = chunk.split("\n").collect();
    let packet1 = Packet::parse(packet_str[0], 0).0;
    let packet2 = Packet::parse(packet_str[1], 0).0;
    all_packets.push(packet1);
    all_packets.push(packet2);

    let n = all_packets.len();
    let ans = cmp(&all_packets[n-2], &all_packets[n-1]);
    if ans == Ordering::Less {
      sum += i + 1;
    }
  }
  println!("{}", sum);

  all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Num(2)])]));
  all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Num(6)])]));
  all_packets.sort_by(cmp);

  let i = all_packets.binary_search_by( |probe| {
    let d1 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    return cmp(probe, &d1);
  }).unwrap();

  let j = all_packets.binary_search_by( |probe| {
    let d2 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    return cmp(probe, &d2);
  }).unwrap();

  println!("{}", (i+1) * (j+1));
}