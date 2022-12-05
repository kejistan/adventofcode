use std::collections::VecDeque;
use std::{io};
use std::io::{BufReader};
use regex::{Regex};

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

struct Move {
  num: usize,
  from: usize,
  to: usize,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let mut stacks: Vec<VecDeque<char>> = Vec::new();
  let mut groups = input.groups();
  let stack_diagram = groups.next().unwrap()?;
  stack_diagram.lines().for_each(|line| {
    line.as_bytes().chunks(4).map(|bytes| {
      if bytes[0] == '[' as u8 {
        Some(bytes[1])
      } else {
        None
      }
    }).enumerate().for_each(|(stack, option)| {
      if let Some(container) = option {
        if stacks.len() < stack + 1 {
          stacks.resize(stack + 1, VecDeque::new());
        }

        stacks[stack].push_front(container as char);
      }
    });
  });

  let r = Regex::new(r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
  let move_text = groups.next().unwrap()?;
  let moves = r.captures_iter(&move_text).map(|captures| {
    Move {
      num: captures.name("num").unwrap().as_str().parse::<usize>().unwrap(),
      from: captures.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1,
      to: captures.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1,
    }
  });

  for Move {num, from, to} in moves {
    for _ in 0..num {
      let container = stacks[from].pop_back().unwrap();
      stacks[to].push_back(container);
    }
  }

  let mut result = "".to_string();
  for stack in stacks {
    result.push(*stack.back().unwrap());
  }

  println!("{}", result);

  Ok(())
}
