use std::cmp::{min, max};
use std::collections::{HashSet};
use std::ops::RangeInclusive;
use std::{io};
use std::io::{BufReader, BufRead};

use coordinate::Coordinate;
use regex::Regex;

mod coordinate;

struct Sensor {
  position: Coordinate,
  radius: i32,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let coordinate_re = Regex::new(r"at x=(-?\d+), y=(-?\d+)").unwrap();

  let mut known_beacons = HashSet::new();
  let sensors = input.lines().map(|result| {
    let line = result.unwrap();
    let mut coordinates = coordinate_re.captures_iter(&line).map(|captures| {
      Coordinate::new(captures.get(1).unwrap().as_str().parse::<i32>().unwrap(), captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
    });
    let position = coordinates.next().unwrap();
    let beacon = coordinates.next().unwrap();

    known_beacons.insert(beacon);

    let distance = position - beacon;

    let radius = distance.x.abs() + distance.y.abs();
    Sensor {
      position,
      radius,
    }
  }).collect::<Vec<Sensor>>();

  let x_range = sensors.iter()
    .map(|sensor| sensor.x_range())
    .reduce(|acc, sensor| {
      min(*acc.start(), *sensor.start())..=max(*acc.end(), *sensor.end())
    }).unwrap();

  let mut result = 0;
  for x in x_range {
    let coordinate = Coordinate::new(x, 2_000_000);
    if known_beacons.contains(&coordinate) {
      continue;
    }

    let contains = sensors.iter().any(|sensor| {
      sensor.contains(&coordinate)
    });

    if contains {
      result += 1;
    }
  }

  println!("{}", result);

  Ok(())
}

impl Sensor {
  fn x_range(&self) -> RangeInclusive<i32> {
    (self.position.x - self.radius)..=(self.position.x + self.radius)
  }

  fn contains(&self, coordinate: &Coordinate) -> bool {
    let distance = self.position - *coordinate;
    self.radius >= distance.x.abs() + distance.y.abs()
  }
}
