use std::collections::VecDeque;
use std::{io};
use std::io::{BufReader};

use crate::grouped_iterator::GroupedIterator;

mod grouped_iterator;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let results = input.groups().map(|result| {
    let pair_str = result.unwrap();
    let mut lines = pair_str.lines();
    let left_packet = tokenize(lines.next().unwrap());
    let right_packet = tokenize(lines.next().unwrap());

    packets_are_in_order(left_packet, right_packet)
  });

  let result: usize = results.enumerate().map(|(i, result)| {
    if result {
      i + 1
    } else {
      0
    }
  }).sum();

  println!("{}", result);

  Ok(())
}

#[derive(Debug, PartialEq)]
enum Token {
  Number(u8),
  ListStart,
  ListEnd,
}

fn tokenize(string: &str) -> VecDeque<Token> {
  let mut tokens = VecDeque::new();
  let mut token_start = 0;
  for (token_end, current_char) in string.char_indices() {
    match current_char {
      ',' | '[' | ']' => {
        if token_start != token_end {
          tokens.push_back(Token::Number(string[token_start..token_end].parse::<u8>().unwrap()));
        }
        match current_char {
          '[' => tokens.push_back(Token::ListStart),
          ']' => tokens.push_back(Token::ListEnd),
          _ => (),
        }
        token_start = token_end + 1;
      },
      _ => (),
    }
  }

  tokens
}

fn packets_are_in_order(mut left_vec: VecDeque<Token>, mut right_vec: VecDeque<Token>) -> bool {
  let mut left = left_vec.pop_front();
  let mut right = right_vec.pop_front();

  while left.is_some() && right.is_some() {
    if left == right {
      left = left_vec.pop_front();
      right = right_vec.pop_front();
      continue;
    }

    if right == Some(Token::ListEnd) {
      return false;
    }
    if left == Some(Token::ListEnd) {
      return true;
    }

    if right == Some(Token::ListStart) {
      left_vec.push_front(Token::ListEnd);
      left_vec.push_front(left.unwrap());
      left = Some(Token::ListStart);
      continue;
    }
    if left == Some(Token::ListStart) {
      right_vec.push_front(Token::ListEnd);
      right_vec.push_front(right.unwrap());
      right = Some(Token::ListStart);
      continue;
    }

    if let Token::Number(l) = left.unwrap() {
      if let Token::Number(r) = right.unwrap() {
        return l < r;
      } else {
        panic!();
      }
    } else {
      panic!();
    }
  }

  !left.is_some()
}
