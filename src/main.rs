use std::collections::{HashSet};
use std::ops::RangeInclusive;
use std::{io};
use std::io::{BufReader, BufRead};

use regex::Regex;

type Coordinate = (i32, i32, i32);

struct Fill {
  map: Vec<bool>,
  x_min: i32,
  x_max: i32,
  y_min: i32,
  y_max: i32,
  y_len: usize,
  z_min: i32,
  z_max: i32,
  z_len: usize,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let regex = Regex::new(r"\d+").unwrap();

  let mut min_x = None;
  let mut max_x = None;
  let mut min_y = None;
  let mut max_y = None;
  let mut min_z = None;
  let mut max_z = None;
  let coordinates = input.lines().map(|result| {
    let line = result.unwrap();

    let mut dims = regex.find_iter(&line).map(|m| {
      m.as_str().parse::<i32>().unwrap()
    });

    let x: i32 = dims.next().unwrap();
    let y: i32 = dims.next().unwrap();
    let z: i32 = dims.next().unwrap();

    if max_x.is_none() || x > max_x.unwrap() {
      max_x = Some(x);
    }
    if min_x.is_none() || x < min_x.unwrap() {
      min_x = Some(x);
    }

    if max_y.is_none() || y > max_y.unwrap() {
      max_y = Some(y);
    }
    if min_y.is_none() || y < min_y.unwrap() {
      min_y = Some(y);
    }

    if max_z.is_none() || z > max_z.unwrap() {
      max_z = Some(z);
    }
    if min_z.is_none() || z < min_z.unwrap() {
      min_z = Some(z);
    }

    (x, y, z)
  }).collect::<HashSet<Coordinate>>();

  let mut fill = Fill::new(
    (min_x.unwrap()-1)..=(max_x.unwrap()+1),
    (min_y.unwrap()-1)..=(max_y.unwrap()+1),
    (min_z.unwrap()-1)..=(max_z.unwrap()+1),
  );

  fill.fill(&coordinates);

  let mut surface_area = 0;
  for coord in coordinates {
    surface_area += count_exposed_surface(coord, &fill);
  }

  println!("{}", surface_area);

  Ok(())
}

fn count_exposed_surface((x, y, z): Coordinate, fill: &Fill) -> i32 {
  let mut surface_area = 0;
  if fill.is_outside((x - 1, y, z)) {
    surface_area += 1;
  }
  if fill.is_outside((x + 1, y, z)) {
    surface_area += 1;
  }
  if fill.is_outside((x, y - 1, z)) {
    surface_area += 1;
  }
  if fill.is_outside((x, y + 1, z)) {
    surface_area += 1;
  }
  if fill.is_outside((x, y, z - 1)) {
    surface_area += 1;
  }
  if fill.is_outside((x, y, z + 1)) {
    surface_area += 1;
  }

  surface_area
}

impl Fill {
  fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>, z_range: RangeInclusive<i32>) -> Fill {
    let x_len = (x_range.end() + 1 - x_range.start()) as usize;
    let y_len = (y_range.end() + 1 - y_range.start()) as usize;
    let z_len = (z_range.end() + 1 - z_range.start()) as usize;

    let len = x_len * y_len * z_len;
    let mut map = Vec::with_capacity(len);
    map.resize(len, false);

    Fill {
      x_min: *x_range.start(),
      x_max: *x_range.end(),
      y_min: *y_range.start(),
      y_max: *y_range.end(),
      y_len,
      z_min: *z_range.start(),
      z_max: *z_range.end(),
      z_len,
      map,
    }
  }

  fn fill(&mut self, coordinates: &HashSet<Coordinate>) {
    let mut stack = Vec::new();
    stack.push((self.x_min, self.y_min, self.z_min));

    while let Some((x, y, z)) = stack.pop() {
      if !coordinates.contains(&(x, y, z)) {
        if self.mark_outside((x, y, z)) {
          if (x - 1) >= self.x_min {
            stack.push((x - 1, y, z));
          }
          if (x + 1) <= self.x_max {
            stack.push((x + 1, y, z));
          }
          if (y - 1) >= self.y_min {
            stack.push((x, y - 1, z));
          }
          if (y + 1) <= self.y_max {
            stack.push((x, y + 1, z));
          }
          if (z - 1) >= self.z_min {
            stack.push((x, y, z - 1));
          }
          if (z + 1) <= self.z_max {
            stack.push((x, y, z + 1));
          }
        }
      }
    }
  }

  fn is_outside(&self, coordinate: Coordinate) -> bool {

    self.map[self.coordinate_to_idx(coordinate)]
  }

  fn mark_outside(&mut self, coordinate: Coordinate) -> bool {
    let idx = self.coordinate_to_idx(coordinate);

    if self.map[idx] {
      return false;
    }

    self.map[idx] = true;

    true
  }

  fn coordinate_to_idx(&self, (x, y, z): Coordinate) -> usize {
    let x_idx = (x - self.x_min) as usize;
    let y_idx = (y - self.y_min) as usize;
    let z_idx = (z - self.z_min) as usize;
    x_idx * self.y_len * self.z_len + y_idx * self.z_len + z_idx
  }
}
