use std::collections::HashSet;
use std::{io};
use std::io::{BufReader, BufRead};

use regex::Regex;

type Coordinate = (i32, i32, i32);

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let regex = Regex::new(r"\d+").unwrap();

  let coordinates = input.lines().map(|result| {
    let line = result.unwrap();

    let mut dims = regex.find_iter(&line).map(|m| {
      m.as_str().parse::<i32>().unwrap()
    });

    (dims.next().unwrap(), dims.next().unwrap(), dims.next().unwrap())
  }).collect::<HashSet<Coordinate>>();

  let mut surface_area = 0;
  for coord in coordinates.iter() {
    if !coordinates.contains(&(coord.0 - 1, coord.1, coord.2)) {
      surface_area += 1;
    }
    if !coordinates.contains(&(coord.0 + 1, coord.1, coord.2)) {
      surface_area += 1;
    }
    if !coordinates.contains(&(coord.0, coord.1 - 1, coord.2)) {
      surface_area += 1;
    }
    if !coordinates.contains(&(coord.0, coord.1 + 1, coord.2)) {
      surface_area += 1;
    }
    if !coordinates.contains(&(coord.0, coord.1, coord.2 - 1)) {
      surface_area += 1;
    }
    if !coordinates.contains(&(coord.0, coord.1, coord.2 + 1)) {
      surface_area += 1;
    }
  }

  println!("{}", surface_area);

  Ok(())
}
