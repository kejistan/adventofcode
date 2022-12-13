use std::collections::VecDeque;
use std::{io};
use std::io::{BufReader, BufRead};

mod grouped_iterator;

#[derive(Eq, PartialEq)]
struct Packet {
  tokens: Vec<Token>,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let packets = input.lines().filter_map(|result| {
    let line = result.unwrap();
    if line == "" {
      return None
    }

    Some(Packet { tokens: tokenize(&line) })
  }).collect::<Vec<Packet>>();

  let mut divider_one_idx = 1;
  let mut divider_two_idx = 2;
  let divider_one = Packet { tokens: tokenize("[[2]]") };
  let divider_two = Packet { tokens: tokenize("[[6]]") };
  for packet in packets {
    if packet < divider_one {
      divider_one_idx += 1;
    }
    if packet < divider_two {
      divider_two_idx += 1;
    }
  }

  let result = divider_one_idx * divider_two_idx;
  println!("{}", result);

  Ok(())
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
  Number(u8),
  ListStart,
  ListEnd,
}

fn tokenize(string: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut token_start = 0;
  for (token_end, current_char) in string.char_indices() {
    match current_char {
      ',' | '[' | ']' => {
        if token_start != token_end {
          tokens.push(Token::Number(string[token_start..token_end].parse::<u8>().unwrap()));
        }
        match current_char {
          '[' => tokens.push(Token::ListStart),
          ']' => tokens.push(Token::ListEnd),
          _ => (),
        }
        token_start = token_end + 1;
      },
      _ => (),
    }
  }

  tokens
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let mut left_queue = VecDeque::from_iter(self.tokens.iter());
    let mut right_queue = VecDeque::from_iter(other.tokens.iter());

    let mut left = left_queue.pop_front();
    let mut right = right_queue.pop_front();
    while left.is_some() && right.is_some() {
      if left == right {
        left = left_queue.pop_front();
        right = right_queue.pop_front();
        continue;
      }
  
      if right == Some(&Token::ListEnd) {
        return Some(std::cmp::Ordering::Greater);
      }
      if left == Some(&Token::ListEnd) {
        return Some(std::cmp::Ordering::Less);
      }
  
      if right == Some(&Token::ListStart) {
        left_queue.push_front(&Token::ListEnd);
        left_queue.push_front(left.unwrap());
        left = Some(&Token::ListStart);
        continue;
      }
      if left == Some(&Token::ListStart) {
        right_queue.push_front(&Token::ListEnd);
        right_queue.push_front(right.unwrap());
        right = Some(&Token::ListStart);
        continue;
      }
  
      if let Token::Number(l) = left.unwrap() {
        if let Token::Number(r) = right.unwrap() {
          return Some(l.cmp(r));
        } else {
          panic!();
        }
      } else {
        panic!();
      }
    }
  
    if left == right {
      return Some(std::cmp::Ordering::Equal);
    }
    if left.is_some() {
      return Some(std::cmp::Ordering::Greater);
    }

    Some(std::cmp::Ordering::Less)
  }
}
