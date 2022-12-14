use std::cmp::{min, max};
use std::collections::{HashSet, VecDeque};
use std::ops::{RangeInclusive};
use std::{io};
use std::io::{BufReader, BufRead};

use regex::Regex;

use crate::coordinate::Coordinate;

mod coordinate;

enum Line {
  Horizontal(RangeInclusive<i32>),
  Vertical(RangeInclusive<i32>),
}
struct CoordinateProducer {
  x: i32,
  y: i32,

  line: Line,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let coord_re = Regex::new(r"(\d+),(\d+)").unwrap();

  let mut occupied_coordinates = input.lines().flat_map(|result| {
    let line = result.unwrap();
    let coordinates = coord_re.captures_iter(&line).map(|captures| {
      let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
      let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

      Coordinate::new(x, y)
    }).collect::<Vec<Coordinate>>();

    coordinates.windows(2).flat_map(|pair| {
      if let [start, end] = pair {
        if start.x == end.x {
          let y_start = min(start.y, end.y);
          let y_end = max(start.y, end.y);
          CoordinateProducer { x: start.x, y: y_start, line: Line::Vertical(y_start..=y_end) }
        } else {
          let x_start = min(start.x, end.x);
          let x_end = max(start.x, end.x);
          CoordinateProducer { x: x_start, y: start.y, line: Line::Horizontal(x_start..=x_end) }
        }
      } else {
        unreachable!();
      }
    }).collect::<Vec<Coordinate>>()
  }).collect::<HashSet<Coordinate>>();

  let max_y = occupied_coordinates.iter().map(|coord| coord.y).max().unwrap() + 1;

  let mut path = VecDeque::new();
  path.push_back(Coordinate::new(500, 0));

  let mut sand_count = 0;
  while !path.is_empty() {
    let coordinate = path.back().unwrap();
    if max_y == coordinate.y {
      sand_count += 1;
      occupied_coordinates.insert(*coordinate);
      path.pop_back();
      continue;
    }

    let mut next_coordinate = Coordinate::new(coordinate.x, coordinate.y + 1);
    if !occupied_coordinates.contains(&next_coordinate) {
      path.push_back(next_coordinate);
      continue;
    }

    next_coordinate.x -= 1;
    if !occupied_coordinates.contains(&next_coordinate) {
      path.push_back(next_coordinate);
      continue;
    }

    next_coordinate.x += 2;
    if !occupied_coordinates.contains(&next_coordinate) {
      path.push_back(next_coordinate);
      continue;
    }
    sand_count += 1;
    occupied_coordinates.insert(*coordinate);
    path.pop_back();
  }

  println!("{}", sand_count);

  Ok(())
}

impl Iterator for CoordinateProducer {
  type Item = Coordinate;

  fn next(&mut self) -> Option<Self::Item> {
    let result;
    match &self.line {
      Line::Horizontal(range) => {
        if range.contains(&self.x) {
          result = Some(Coordinate::new(self.x, self.y));
          self.x += 1;
        } else {
          result = None;
        }
      },
      Line::Vertical(range) => {
        if range.contains(&self.y) {
          result = Some(Coordinate::new(self.x, self.y));
          self.y += 1;
        } else {
          result = None;
        }
      },
    }

    result
  }
}
