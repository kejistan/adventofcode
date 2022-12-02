use core::panic;
use std::{io};
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone, Copy)]
enum Shape {
  Rock = 1,
  Paper,
  Scissors,
}

struct Round {
  opponent: Shape,
  me: Shape,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let rounds = input.lines().map(|line| {
    let line_string = line.unwrap();
    let mut round_shapes = line_string.split_ascii_whitespace().take(2).map(|str| match str {
      "A" => Shape::Rock,
      "B" => Shape::Paper,
      "C" => Shape::Scissors,
      "X" => Shape::Rock,
      "Y" => Shape::Paper,
      "Z" => Shape::Scissors,
      _ => panic!(),
    });

    Round {
      opponent: round_shapes.next().unwrap(),
      me: round_shapes.next().unwrap(),
    }
  });

  let score: u32 = rounds.map(|round| round.score()).sum();

  println!("{}", score);

  Ok(())
}

impl Round {
  fn score(&self) -> u32 {
    let outcome = match self.me {
      x if x == self.opponent => 3,
      Shape::Rock if self.opponent == Shape::Paper => 0,
      Shape::Paper if self.opponent == Shape::Scissors => 0,
      Shape::Scissors if self.opponent == Shape::Rock => 0,
      _ => 6,
    };

    outcome + (self.me as u32)
  }
}