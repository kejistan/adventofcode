use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Token {
  Num(u64),
  Sum,
  Mul,
  Open,
  Close,
}

use Token::*;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
 
  let mut result = 0;
  for l in reader.lines() {
    let tokens = tokenize(&l.unwrap());
    let (num, _) = parse(&tokens);

    result += num;
  }

  println!("result: {}", result);

  Ok(())
}

fn parse<'a>(mut tokens: &'a [Token]) -> (u64, &'a [Token]) {
  let mut result = 0;
  let mut operation: Option<Token> = None;

  while !tokens.is_empty() {
    match tokens[0] {
      Close => {
        return (result, &tokens[1..])
      },
      Open => {
        let (num, new_tokens) = parse(&tokens[1..]);
        if let Some(op) = operation {
          result = perform(result, &op, num);
          operation = None;
        } else {
          result = num;
        }
        tokens = new_tokens;
      },
      Sum => {
        operation = Some(Sum);
        tokens = &tokens[1..];
      },
      Mul => {
        operation = Some(Mul);
        tokens = &tokens[1..];
      },
      Num(num) => {
        if let Some(op) = operation {
          result = perform(result, &op, num);
          operation = None;
        } else {
          result = num;
        }

        tokens = &tokens[1..];
      }
    }
  }

  (result, &[])
}

fn perform(left: u64, operation: &Token, right: u64) -> u64 {
  match operation {
    Sum => left + right,
    Mul => left * right,
    _ => panic!(),
  }
}

fn tokenize(string: &str) -> Vec<Token> {
  let mut number_start: Option<usize> = None;
  let mut tokens = Vec::new();

  for (i, character) in string.char_indices() {
    match character {
      ' ' => if let Some(start) = number_start {
        tokens.push(Num(string[start..i].parse().unwrap()));
        number_start = None;
      },
      ')' => {
        if let Some(start) = number_start {
          tokens.push(Num(string[start..i].parse().unwrap()));
          number_start = None;
        }
        tokens.push(Close);
      },
      '(' => tokens.push(Open),
      '+' => tokens.push(Sum),
      '*' => tokens.push(Mul),
      _ => if number_start == None {
        number_start = Some(i);
      },
    }
  }

  if let Some(num) = number_start {
    tokens.push(Num(string[num..].parse().unwrap()));
  }
  
  tokens
}
