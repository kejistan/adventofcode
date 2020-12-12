use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Direction {
  North,
  East,
  South,
  West,
  Left,
  Right,
  Forward,
}


fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let instructions = reader.lines().map(|l| {
    let line = l.unwrap();
    let characters = line.as_str();
    let direction = match &characters[0..1] {
      "N" => Direction::North,
      "E" => Direction::East,
      "S" => Direction::South,
      "W" => Direction::West,
      "L" => Direction::Left,
      "R" => Direction::Right,
      "F" => Direction::Forward,
      _ => panic!(),
    };

    let value = characters[1..].parse::<i32>().unwrap();

    (direction, value)
  });

  let (_, pos) = instructions.fold((0 as i32, (0 as i32, 0 as i32)), |(heading, pos), (dir, value)| {
    let mut direction = dir;
    if direction == Direction::Forward {
      direction = match heading.rem_euclid(360) {
        0 => Direction::East,
        90 => Direction::North,
        180 => Direction::West,
        270 => Direction::South,
        _ => panic!("{}", heading),
      }
    }

    match direction {
      Direction::North => (heading, (value + pos.0, pos.1)),
      Direction::East => (heading, (pos.0, value + pos.1)),
      Direction::South => (heading, (pos.0 - value, pos.1)),
      Direction::West => (heading, (pos.0, pos.1 - value)),
      Direction::Left => (heading + value, pos),
      Direction::Right => (heading - value, pos),
      Direction::Forward => panic!(),
    }
  });

  let distance = pos.0.abs() + pos.1.abs();
  println!("{}", distance);

  Ok(())
}
