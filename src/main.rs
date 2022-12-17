use std::cmp::max;
use std::{io};
use std::io::{BufReader, Read};

use coordinate::Coordinate;

mod coordinate;

type Coord = Coordinate<i64>;
type Row = [bool; 7];

enum Direction {
  Left,
  Right,
}

struct Shape {
  coordinates: Vec<Coord>,
}

struct Tunnel {
  column_heights: [i64; 7],
  floor: i64,
  occupied: Vec<Row>,
}

struct HistoryEntry {
  occupied: Vec<Row>,
  floor: i64,
  rock_count: u64,
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

  let mut history: Vec<Vec<HistoryEntry>> = Vec::with_capacity(all_directions.len());
  history.resize_with(all_directions.len(), || Vec::new());

  let mut rock_count: u64 = 0;
  let rock_limit = 1000000000000u64;

  let mut maybe_repeat = None;
  println!("Finding pattern");
  'find_pattern: while rock_count < rock_limit {
    add_rock(&mut shapes, &mut directions, &mut tunnel);
    rock_count += 1;

    if shapes.idx == 0 {
      let this_entry = HistoryEntry::new(&tunnel, rock_count);
      let history_entries = &mut history[directions.idx];
      for history_entry in history_entries.iter() {
        if *history_entry == this_entry {
          let height_difference = this_entry.floor - history_entry.floor;
          let rock_difference = this_entry.rock_count - history_entry.rock_count;
          maybe_repeat = Some((rock_difference, height_difference));
          break 'find_pattern;
        }
      }

      history_entries.push(this_entry);
    }

    if rock_count % (1000000000000u64 / 1000) == 0 {
      let pct = rock_count / (1000000000000u64 / 1000);
      println!("{}.{}%", pct / 10, pct % 10);
    }
  }

  if rock_count == rock_limit {
    println!("No repeats found before finishing simulation");
    println!("{}", tunnel.height());

    return Ok(());
  }

  let (repeat_rocks, repeat_height) = maybe_repeat.unwrap();
  println!("Found pattern after {} rocks", rock_count);
  println!("Pattern repeats every {} rocks and adds {} height", repeat_rocks, repeat_height);

  let skippable_repeats = (rock_limit - rock_count) / repeat_rocks;
  let rocks_skipped = skippable_repeats * repeat_rocks;
  let height_skipped = skippable_repeats as i64 * repeat_height;
  println!("Skipping ahead {} rocks (+{} height)", rocks_skipped, height_skipped);

  tunnel.floor += height_skipped;
  for col in tunnel.column_heights.iter_mut() {
    *col += height_skipped;
  }
  rock_count += rocks_skipped;

  println!("Simulating remaining rocks");
  while rock_count < rock_limit {
    add_rock(&mut shapes, &mut directions, &mut tunnel);
    rock_count += 1;

    if rock_count % (1000000000000u64 / 1000) == 0 {
      let pct = rock_count / (1000000000000u64 / 1000);
      println!("{}.{}%", pct / 10, pct % 10);
    }
  }

  println!("{}", tunnel.height());

  Ok(())
}

impl HistoryEntry {
  fn new(tunnel: &Tunnel, rock_count: u64) -> HistoryEntry {
    let occupied = tunnel.occupied.clone();
    HistoryEntry { occupied, floor: tunnel.floor, rock_count }
  }
}

impl PartialEq for HistoryEntry {
  fn eq(&self, other: &Self) -> bool {
    self.occupied.eq(&other.occupied)
  }
}

fn add_rock(shapes: &mut dyn Iterator<Item = &Shape>, directions: &mut dyn Iterator<Item = &Direction>, tunnel: &mut Tunnel) {
  let mut coordinate = tunnel.start_coordinate();
  let rock = shapes.next().unwrap();

  loop {
    coordinate = handle_jet(&rock, coordinate, directions, tunnel);
    if let Some(coord) = fall(&rock, coordinate, tunnel) {
      coordinate = coord;
    } else {
      break;
    }
  }
}

fn handle_jet(rock: &Shape, mut coordinate: Coord, directions: &mut dyn Iterator<Item = &Direction>, tunnel: &Tunnel) -> Coord {
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

fn fall(rock: &Shape, mut coordinate: Coord, tunnel: &mut Tunnel) -> Option<Coord> {
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
    Tunnel { column_heights: [0; 7], floor: 0, occupied: Vec::new() }
  }

  fn start_coordinate(&self) -> Coord {
    Coordinate::new(2, self.height() + 3)
  }

  fn can_place_at(&self, position: &Coord, shape: &Shape) -> bool {
    let floor = self.floor();
    if position.y < floor {
      return false;
    }

    !shape.coordinates.iter()
      .map(|offset| position + offset)
      .any(|coordinate| coordinate.x < 0 || coordinate.x >= self.column_heights.len() as i64 || self.is_occupied(&coordinate))
  }

  fn settle_at(&mut self, position: &Coord, shape: &Shape) {
    let mut new_floor = None;
    for coordinate in shape.coordinates.iter().map(|offset| position + offset) {
      let height = coordinate.y + 1;
      self.column_heights[coordinate.x as usize] = max(self.column_heights[coordinate.x as usize], height);

      self.mark_occupied(&coordinate);

      let is_new_floor = !(0..self.column_heights.len()).map(|x| Coordinate::new(x as i64, coordinate.y)).any(|coord| !self.is_occupied(&coord));
      if is_new_floor {
        new_floor = max(Some(coordinate.y), new_floor);
      }
    }

    if let Some(floor) = new_floor {
      self.set_floor(floor);
    }
  }

  fn set_floor(&mut self, floor: i64) {
    let y: usize = floor as usize - self.floor() as usize;
    self.occupied.copy_within(y.., 0);
    self.occupied.truncate(self.occupied.len() - y);
    self.floor = floor;
  }

  fn mark_occupied(&mut self, position: &Coord) {
    let y: usize = position.y as usize - self.floor() as usize;
    let x: usize = position.x as usize;
    if y >= self.occupied.len() {
      self.occupied.resize(y + 1, [false; 7]);
    }
    self.occupied[y][x] = true;
  }

  fn is_occupied(&self, position: &Coord) -> bool {
    let y: usize = position.y as usize - self.floor() as usize;
    let x: usize = position.x as usize;
    if y < self.occupied.len() {
      self.occupied[y][x]
    } else {
      false
    }
  }

  fn floor(&self) -> i64 {
    self.floor
  }

  fn height(&self) -> i64 {
    *self.column_heights.iter().max().unwrap()
  }
}
