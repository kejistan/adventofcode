use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
enum Token {
  Num(u64),
  Sum,
  Mul,
  Open,
  Close,
}

use Token::*;

#[derive(Debug)]
enum Expression {
  Num(u64),
  Sum(Box<Expression>, Box<Expression>),
  Mul(Box<Expression>, Box<Expression>),
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
 
  let mut result = 0;
  for l in reader.lines() {
    let tokens = tokenize(&l.unwrap());
    let expression = parse(&tokens);
    result += evaluate(expression);
  }

  println!("result: {}", result);

  Ok(())
}

fn evaluate(expression: Expression) -> u64 {
  match expression {
    Expression::Num(num) => num,
    Expression::Mul(l, r) => evaluate(*l) * evaluate(*r),
    Expression::Sum(l, r) => evaluate(*l) + evaluate(*r),
  }
}

fn parse_num<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  match tokens[0] {
    Num(num) => {
      (Expression::Num(num), &tokens[1..])
    },
    _ => panic!("{:?}", tokens[0]),
  }
}

fn parse_unit<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  match tokens[0] {
    Num(_) => parse_num(tokens),
    Open => parse_paren(tokens),
    _ => panic!("{:?}", tokens[0]),
  }
}

fn parse_paren<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  if tokens[0] != Open {
    panic!("{:?}", tokens[0]);
  }

  let (expr, more_tokens) = parse_expression(&tokens[1..]);

  if more_tokens[0] != Close {
    panic!("{:?}", more_tokens[0]);
  }

  (expr, &more_tokens[1..])
}

fn parse_sum<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  let (mut left, mut more_tokens) = parse_unit(tokens);

  while !more_tokens.is_empty() && more_tokens[0] == Sum {
    let (right, t) = parse_unit(&more_tokens[1..]);
    left = Expression::Sum(Box::new(left), Box::new(right));
    more_tokens = t;
  }

  (left, more_tokens)
}

fn parse_mul<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  let (mut left, mut more_tokens) = parse_sum(tokens);

  while !more_tokens.is_empty() && more_tokens[0] == Mul {
    let (right, t) = parse_sum(&more_tokens[1..]);
    left = Expression::Mul(Box::new(left), Box::new(right));
    more_tokens = t;
  }

  (left, more_tokens)
}

fn parse_expression<'a>(tokens: &'a [Token]) -> (Expression, &'a [Token]) {
  parse_mul(tokens)
}

fn parse<'a>(tokens: &'a [Token]) -> Expression {
  let (expr, more_tokens) = parse_expression(tokens);
  if !more_tokens.is_empty() {
    panic!("{:?}", more_tokens);
  }

  expr
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
