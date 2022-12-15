use std::collections::{HashSet};
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

  let mut open_space = Coordinate::new(0, 0);
  while !is_unoccupied(&sensors, &open_space) {
    let next = next_unoccupied(&sensors, &open_space);
    if next.x >= 4_000_000 {
      open_space.y += 1;
      open_space.x = 0;
    } else {
      open_space = next;
    }
  }

  let result = open_space;

  println!("{}", result.x as u64 * 4_000_000 + result.y as u64);

  Ok(())
}

fn is_unoccupied(sensors: &Vec<Sensor>, coordinate: &Coordinate) -> bool {
  !sensors.iter().any(|sensor| sensor.contains(coordinate))
}

fn next_unoccupied(sensors: &Vec<Sensor>, coordinate: &Coordinate) -> Coordinate {
  let max_coordinate = sensors.iter().filter_map(|sensor| {
    if sensor.contains(&coordinate) {
      let available_distance = sensor.radius - (sensor.position.y - coordinate.y).abs();
      Some(Coordinate::new(sensor.position.x + available_distance, coordinate.y))
    } else {
      None
    }
  }).max_by(|a, b| {
    a.x.cmp(&b.x)
  });

  max_coordinate.unwrap_or(*coordinate) + Coordinate::new(1, 0)
}

impl Sensor {
  fn contains(&self, coordinate: &Coordinate) -> bool {
    let distance = self.position - *coordinate;
    self.radius >= distance.x.abs() + distance.y.abs()
  }
}
