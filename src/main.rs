mod coordinate;

use std::collections::HashSet;
use std::{io};
use std::io::{BufReader, BufRead};

use crate::coordinate::Coordinate;

enum Motion {
  Right(u8),
  Left(u8),
  Up(u8),
  Down(u8),
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let motions = input.lines().map(|result| {
    let line = result.unwrap();
    let (command, number) = line.split_at(2);
    let count = number.parse::<u8>().unwrap();
    match command {
      "L " => Motion::Left(count),
      "R " => Motion::Right(count),
      "U " => Motion::Up(count),
      "D " => Motion::Down(count),
      _ => unreachable!(),
    }
  });

  let mut tail_positions: HashSet<Coordinate> = HashSet::new();
  let mut knots = [Coordinate::new(0, 0); 10];

  tail_positions.insert(knots[9]);

  for motion in motions {
    let (delta, count) = match motion {
      Motion::Down(count) => (Coordinate::new(0, -1), count),
      Motion::Up(count) => (Coordinate::new(0, 1), count),
      Motion::Left(count) => (Coordinate::new(-1, 0), count),
      Motion::Right(count) => (Coordinate::new(1, 0), count),
    };

    for _ in 0..count {
      knots[0] += delta;

      for i in 1..knots.len() {
        let (head, tail) = knots.split_at_mut(i);
        update_knot(&head[i - 1], &mut tail[0]);
      }
      tail_positions.insert(knots[9]);
    }
  }

  let result = tail_positions.len();
  println!("{}", result);

  Ok(())
}

fn update_knot(head: &Coordinate, tail: &mut Coordinate) {
  let delta = head - tail;
  let mut correction = Coordinate::new(0, 0);
  if delta.x.abs() > 1 || delta.y.abs() > 1 {
    if delta.x > 0 {
      correction.x = 1;
    } else if delta.x < 0 {
      correction.x = -1;
    }
    if delta.y > 0 {
      correction.y = 1;
    } else if delta.y < 0 {
      correction.y = -1;
    }
  }

  *tail += correction;
}
