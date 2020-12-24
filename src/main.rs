use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use regex::Regex;

#[derive(Debug)]
enum Direction {
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest,
  NorthEast,
}

use Direction::*;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let instruction_regex = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();

  let instructions = input.lines().map(|l| {
    let line = l.unwrap();
    instruction_regex.captures_iter(&line).map(|cap| {
      match &cap[1] {
        "e" => East,
        "se" => SouthEast,
        "sw" => SouthWest,
        "w" => West,
        "nw" => NorthWest,
        "ne" => NorthEast,
        other => panic!("{}", other),
      }
    }).collect::<Vec<Direction>>()
  });

  let mut black_coords = HashSet::new();
  for coord in instructions.map(traverse) {
    if black_coords.contains(&coord) {
      black_coords.remove(&coord);
    } else {
      black_coords.insert(coord);
    }
  }

  println!("{}", black_coords.len());

  Ok(())
}

fn traverse(instruction: Vec<Direction>) -> (i32, i32) {
  let mut x = 0;
  let mut y = 0;
  for i in instruction {
    match i {
      East => x += 1,
      SouthEast => {
        x += 1;
        y -= 1;
      },
      SouthWest => y -= 1,
      West => x -= 1,
      NorthWest => {
        x -= 1;
        y += 1;
      },
      NorthEast => y += 1,
    }
  }

  (x, y)
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string = match self {
      East => "e",
      SouthEast => "se",
      SouthWest => "sw",
      West => "w",
      NorthWest => "nw",
      NorthEast => "ne",
    };
    write!(f, "{}", string)
  }
}
