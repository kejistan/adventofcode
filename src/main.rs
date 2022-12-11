use std::collections::VecDeque;
use std::{io};
use std::io::{BufReader};

use regex::Regex;

use crate::grouped_iterator::GroupedIterator;

mod grouped_iterator;

enum Operation {
  Add(u8),
  Mul(u8),
  AddSelf,
  MulSelf,
}

struct Item {
  worry_level: usize,
}

struct Test {
  divisible_by: u8,
  if_true: usize,
  if_false: usize,
}

struct Monkey {
  items: VecDeque<Item>,
  operation: Operation,
  test: Test,
  inspect_count: usize,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let items_regex = Regex::new(r"(\d+)").unwrap();
  let operation_regex = Regex::new(r"new = old (\+|\*) (old|\d+)").unwrap();
  let divisible_by_regex = Regex::new(r"divisible by (\d+)").unwrap();
  let throw_regex = Regex::new(r"throw to monkey (\d+)").unwrap();

  let mut monkeys = input.groups().map(|result| {
      let string = result.unwrap();
      let mut lines = string.lines().skip(1);
      let items = items_regex.captures_iter(lines.next().unwrap())
        .map(|capture| Item { worry_level: capture.get(1).unwrap().as_str().parse::<usize>().unwrap() })
        .collect::<VecDeque<Item>>();
      let operation_capture = operation_regex.captures(lines.next().unwrap()).unwrap();
      let operation;
      if operation_capture.get(2).unwrap().as_str() == "old" {
        operation = match operation_capture.get(1).unwrap().as_str() {
          "*" => Operation::MulSelf,
          "+" => Operation::AddSelf,
          _ => unreachable!(),
        };
      } else {
        operation = match operation_capture.get(1).unwrap().as_str() {
          "*" => Operation::Mul(operation_capture.get(2).unwrap().as_str().parse::<u8>().unwrap()),
          "+" => Operation::Add(operation_capture.get(2).unwrap().as_str().parse::<u8>().unwrap()),
          _ => unreachable!(),
        };
      }
      let divisible_by = divisible_by_regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<u8>().unwrap();
      let if_true = throw_regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
      let if_false = throw_regex.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
      let test = Test { divisible_by, if_true, if_false };
      
      Monkey { items, operation, test, inspect_count: 0 }
  }).collect::<Vec<Monkey>>();

  let monkey_worry_mod = monkeys.iter().fold(1, |acc, monkey| {
    acc * monkey.test.divisible_by as usize
  });

  let mut item_inspect_counts = Vec::with_capacity(monkeys.len());
  for _ in 0..monkeys.len() {
    item_inspect_counts.push(0);
  }

  for _ in 0..10_000 {
    for i in 0..monkeys.len() {
      let monkey = &mut monkeys[i];

      for (mut item, target) in monkey.inspect_items() {
        item.worry_level %= monkey_worry_mod;
        monkeys[target].items.push_back(item);
      }
    }
  }

  monkeys.sort_unstable_by(|a, b| {
    b.inspect_count.cmp(&a.inspect_count)
  });

  let result: usize = monkeys[0..2].iter().map(|monkey| monkey.inspect_count).product();
  println!("{}", result);

  Ok(())
}

impl Monkey {
  fn inspect_items(&mut self) -> Vec<(Item, usize)> {
    self.items.drain(..).map(|mut item| {
      self.operation.apply(&mut item);
      self.inspect_count += 1;

      let target = self.test.apply(&item);
      (item, target)
    }).collect()
  }
}

impl Operation {
  fn apply(&self, item: &mut Item) {
    match self {
      Operation::Add(arg) => item.worry_level += *arg as usize,
      Operation::Mul(arg) => item.worry_level *= *arg as usize,
      Operation::AddSelf => item.worry_level += item.worry_level,
      Operation::MulSelf => item.worry_level *= item.worry_level,
    }
  }
}

impl Test {
  fn apply(&self, item: &Item) -> usize {
    if item.worry_level % self.divisible_by as usize == 0 {
      self.if_true
    } else {
      self.if_false
    }
  }
}
