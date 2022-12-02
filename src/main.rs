use core::panic;
use std::{io};
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Clone, Copy)]
enum Shape {
  Rock = 1,
  Paper,
  Scissors,
}

enum Outcome {
  Win,
  Draw,
  Lose,
}

struct Round {
  opponent: Shape,
  outcome: Outcome,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let rounds = input.lines().map(|line| {
    let line_string = line.unwrap();
    let mut letters = line_string.split_ascii_whitespace().take(2);
  
    Round {
      opponent: match letters.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!(),
      },
      outcome: match letters.next().unwrap() {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!(),
      },
    }
  });

  let score: u32 = rounds.map(|round| round.score()).sum();

  println!("{}", score);

  Ok(())
}

impl Round {
  fn score(&self) -> u32 {
    match self.outcome {
      Outcome::Draw => 3 + self.opponent as u32,
      Outcome::Lose => match self.opponent {
        Shape::Rock => Shape::Scissors as u32,
        Shape::Paper => Shape::Rock as u32,
        Shape::Scissors => Shape::Paper as u32,
      }
      Outcome::Win => 6 + match self.opponent {
        Shape::Rock => Shape::Paper as u32,
        Shape::Paper => Shape::Scissors as u32,
        Shape::Scissors => Shape::Rock as u32,
      }
    }
  }
}