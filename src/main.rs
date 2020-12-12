use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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

  let (_, pos) = instructions.fold(((10, 1), (0, 0)), |(waypoint, pos), (direction, value)| {
    match direction {
      Direction::North => ((waypoint.0, waypoint.1 + value), pos),
      Direction::East => ((waypoint.0 + value, waypoint.1), pos),
      Direction::South => ((waypoint.0, waypoint.1 - value), pos),
      Direction::West => ((waypoint.0 - value, waypoint.1), pos),
      Direction::Left => (rotate(waypoint, value), pos),
      Direction::Right => (rotate(waypoint, -value), pos),
      Direction::Forward => (waypoint, (pos.0 + value * waypoint.0, pos.1 + value * waypoint.1)),
    }
  });

  let distance = pos.0.abs() + pos.1.abs();
  println!("{}", distance);

  Ok(())
}

fn rotate(pos: (i32, i32), degrees: i32) -> (i32, i32) {
  let (x, y) = pos;
  match degrees.rem_euclid(360) {
    0 => (x, y),
    90 => (-y, x),
    180 => (-x, -y),
    270 => (y, -x),
    _ => panic!("{}", degrees),
  }
}
