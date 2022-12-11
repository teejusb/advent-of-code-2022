use std::collections::{HashMap, VecDeque};
use std::fs;

struct Monkey {
  q: VecDeque<i64>,
  op: Box<dyn Fn(i64) -> i64>,
  test: Box<dyn Fn(i64) -> usize>,
  test_divisor: i64,
  count: i64,
}

impl Monkey {
  fn new() -> Monkey {
    return Monkey{
      q: VecDeque::new(),
      op: Box::new(|_| 0),
      test: Box::new(|_| 0),
      test_divisor: 1,
      count: 0};
  }

  fn add(&mut self, val: i64) {
    self.q.push_back(val);
  }

  fn eval(&mut self, worry_mod: Box<dyn Fn(i64) -> i64>) -> HashMap<usize, Vec<i64>> {
    let mut changes: HashMap<usize, Vec<i64>> = HashMap::new();
    while !self.q.is_empty() {
      let mut val = self.q.pop_front().unwrap();
      val = worry_mod((self.op)(val));
      self.count += 1;
      let idx = (self.test)(val);
      changes.entry(idx).or_default().push(val);
    }
    return changes;
  }
}

fn make_operation(operator: &str, operand_str: &str)
    -> Box<dyn Fn(i64) -> i64> {
  if operand_str == "old" {
    if operator == "*" {
      return Box::new(|x| x * x);
    } else {
      return Box::new(|x| x + x);
    }
  } else {
    let operand: i64 = operand_str.parse().unwrap();
    if operator == "*" {
      return Box::new(move |x| x * operand);
    } else {
      return Box::new(move |x| x + operand);
    }
  }
}

fn make_test(divisor: i64, if_true: usize, if_false: usize)
    -> Box<dyn Fn(i64) -> usize> {
  return Box::new(move |x| {
    if x % divisor == 0 {
      return if_true;
    } else {
      return if_false;
    }
  })
}

pub fn solve() {
  let contents = fs::read_to_string("src/days/day11/in.txt")
                    .expect("Cannot open file").replace("\r\n", "\n");
  // Idk how to create a clone trait on a struct with Boxed members.
  // Just create two separate vectors for part 1 and part 2.
  let mut monkeys1: Vec<Monkey> = Vec::new();
  let mut monkeys2: Vec<Monkey> = Vec::new();
  
  for chunk in contents.split("\n\n") {
    monkeys1.push(Monkey::new());
    monkeys2.push(Monkey::new());
    let lines: Vec<&str> = chunk.split("\n").collect();
    let mut monkey1 = monkeys1.last_mut().unwrap();
    let mut monkey2 = monkeys2.last_mut().unwrap();

    // Parse the items.
    let items: Vec<i64> =
        lines[1].trim().split(": ").last().unwrap()
                .split(", ").map(|x| x.trim().parse().unwrap()).collect();
    for item in items.iter() {
      monkey1.add(*item);
      monkey2.add(*item);
    }

    // Parse operation.
    let parts: Vec<&str> =
        lines[2].trim().split("= old ").last().unwrap()
                .split(" ").collect();
    monkey1.op = make_operation(parts[0], parts[1]);
    monkey2.op = make_operation(parts[0], parts[1]);

    // Parse test.
    let divisor: i64 =
        lines[3].trim().split("divisible by ").last().unwrap().parse().unwrap();
    let if_true: usize =
        lines[4].trim().split("to monkey ").last().unwrap().parse().unwrap();
    let if_false: usize =
        lines[5].trim().split("to monkey ").last().unwrap().parse().unwrap();
    monkey1.test_divisor = divisor;
    monkey1.test = make_test(divisor, if_true, if_false);
    monkey2.test_divisor = divisor;
    monkey2.test = make_test(divisor, if_true, if_false);
  }

  // Part 1
  for _ in 0..20 {
    for i in 0..monkeys1.len() {
      let monkey = &mut monkeys1[i];
      let changes = monkey.eval(Box::new(|x| x / 3));
      for (idx, v) in changes.iter() {
        monkeys1[*idx].q.extend(v);
      }
    }
  }

  monkeys1.sort_by(|a, b| b.count.cmp(&a.count));
  println!("{}", monkeys1[0].count * monkeys1[1].count);

  // Part 2
  // Take the product of divisors to determine a valid period to prevent
  // overflow. The input has prime divisors so don't need to decompose further.
  let mut modulus_product = 1;
  for monkey in monkeys2.iter() {
    modulus_product *= monkey.test_divisor;
  }
  for _ in 0..10000 {
    for i in 0..monkeys2.len() {
      let monkey = &mut monkeys2[i];
      let changes = monkey.eval(Box::new(move |x| x % modulus_product));
      for (idx, v) in changes.iter() {
        monkeys2[*idx].q.extend(v);
      }
    }
  }

  monkeys2.sort_by(|a, b| b.count.cmp(&a.count));
  println!("{}", monkeys2[0].count * monkeys2[1].count);
}