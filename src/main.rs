use std::collections::HashSet;
use std::{io};
use std::io::{BufReader, Read};

use coordinate::Coordinate;

mod coordinate;

enum Direction {
  Left,
  Right,
}

struct Shape {
  coordinates: Vec<Coordinate>,
}

struct Tunnel {
  height: i32,
  occupied: HashSet<Coordinate>,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let all_directions = input.bytes().map(|result| match result.unwrap() as char {
    '<' => Direction::Left,
    '>' => Direction::Right,
    _ => unreachable!(),
  }).collect::<Vec<Direction>>();

  let all_shapes = vec![
    Shape { coordinates: vec![
      Coordinate::new(0, 0),
      Coordinate::new(1, 0),
      Coordinate::new(2, 0),
      Coordinate::new(3, 0),
    ], },
    Shape { coordinates: vec![
      Coordinate::new(1, 0),
      Coordinate::new(0, 1),
      Coordinate::new(1, 1),
      Coordinate::new(2, 1),
      Coordinate::new(1, 2),
    ], },
    Shape { coordinates: vec![
      Coordinate::new(0, 0),
      Coordinate::new(1, 0),
      Coordinate::new(2, 0),
      Coordinate::new(2, 1),
      Coordinate::new(2, 2),
    ], },
    Shape { coordinates: vec![
      Coordinate::new(0, 0),
      Coordinate::new(0, 1),
      Coordinate::new(0, 2),
      Coordinate::new(0, 3),
    ], },
    Shape { coordinates: vec![
      Coordinate::new(0, 0),
      Coordinate::new(1, 0),
      Coordinate::new(0, 1),
      Coordinate::new(1, 1),
    ], },
  ];

  let mut tunnel = Tunnel::new();
  let mut directions = all_directions.cycle();
  let mut shapes = all_shapes.cycle();

  for _ in 0..2022 {
    let rock = shapes.next().unwrap();
    let mut coordinate = tunnel.start_coordinate();

    loop {
      coordinate = handle_jet(&rock, coordinate, &mut directions, &tunnel);
      if let Some(coord) = fall(&rock, coordinate, &mut tunnel) {
        coordinate = coord;
      } else {
        break;
      }
    }
  }


  let result = tunnel.height;

  println!("{}", result);

  Ok(())
}

fn handle_jet(rock: &Shape, mut coordinate: Coordinate, directions: &mut dyn Iterator<Item = &Direction>, tunnel: &Tunnel) -> Coordinate {
  let direction = directions.next().unwrap();
  match direction {
    Direction::Left => coordinate.x -= 1,
    Direction::Right => coordinate.x += 1,
  }

  if !tunnel.can_place_at(&coordinate, rock) {
    match direction {
      Direction::Left => coordinate.x += 1,
      Direction::Right => coordinate.x -= 1,
    }
  }

  coordinate
}

fn fall(rock: &Shape, mut coordinate: Coordinate, tunnel: &mut Tunnel) -> Option<Coordinate> {
  coordinate.y -= 1;

  if tunnel.can_place_at(&coordinate, rock) {
    return Some(coordinate);
  }

  coordinate.y += 1;
  tunnel.settle_at(&coordinate, rock);
  None
}

trait CycleIterator<T> {
  fn cycle(&self) -> Cycler<T>;
}

impl<'a, T> CycleIterator<T> for Vec<T> {
  fn cycle(&self) -> Cycler<T> {
    Cycler { vec: &self, idx: 0 }
  }
}

struct Cycler<'a, T> {
  vec: &'a Vec<T>,
  idx: usize,
}

impl<'a, T> Iterator for Cycler<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.vec.is_empty() {
      return None;
    }
    let result = Some(&self.vec[self.idx]);
    self.idx = (self.idx + 1) % self.vec.len();

    result
  }
}

impl Tunnel {
  fn new() -> Tunnel {
    Tunnel { height: 0, occupied: HashSet::new() }
  }

  fn start_coordinate(&self) -> Coordinate {
    Coordinate::new(2, self.height + 3)
  }

  fn can_place_at(&self, coordinate: &Coordinate, shape: &Shape) -> bool {
    !shape.coordinates.iter()
      .map(|offset| coordinate + offset)
      .any(|coordinate| self.occupied.contains(&coordinate) || coordinate.y < 0 || coordinate.x < 0 || coordinate.x >= 7)
  }

  fn settle_at(&mut self, coordinate: &Coordinate, shape: &Shape) {
    for coordinate in shape.coordinates.iter().map(|offset| coordinate + offset) {
      if self.height < coordinate.y + 1 {
        self.height = coordinate.y + 1;
      }
      self.occupied.insert(coordinate);
    }
  }
}
