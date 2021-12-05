use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::max;
use std::cmp::min;
use std::mem::swap;
use regex::Regex;
use core::iter::Iterator;

fn main() -> io::Result<()> {
  let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
  let input = BufReader::new(io::stdin());

  let mut map = HashMap::new();

  let iter = input.lines().map(|l| {
    let line = l.unwrap();
    let cap = re.captures(&line).unwrap();
    let x1 = cap["x1"].parse::<u32>().unwrap();
    let y1 = cap["y1"].parse::<u32>().unwrap();
    let x2 = cap["x2"].parse::<u32>().unwrap();
    let y2 = cap["y2"].parse::<u32>().unwrap();

    ((x1, y1), (x2, y2))
  });

  for line in iter {
    draw_line(&mut map, line);
  }

  let count = map.values().filter(|count| **count > 1).count();

  println!("{}", count);

  Ok(())
}

fn draw_line(map: &mut HashMap<(u32, u32), u32>, (mut start, mut end): ((u32, u32), (u32, u32))) {
  if start.0 == end.0 {
    for y in min(start.1, end.1)..=max(start.1, end.1) {
      let count = map.entry((start.0, y)).or_insert(0);
      *count += 1;
    }
  } else if start.1 == end.1 {
    for x in min(start.0, end.0)..=max(start.0, end.0) {
      let count = map.entry((x, start.1)).or_insert(0);
      *count += 1;
    }
  } else {
    if start.0 > end.0 {
      swap(&mut start, &mut end);
    }

    let slope: i32 = if start.1 > end.1 {
      -1
    } else {
      1
    };

    for i in 0..=(end.0 - start.0) {
      let x = i + start.0;
      let y = (i as i32 * slope + start.1 as i32) as u32;

      let count = map.entry((x, y)).or_insert(0);
      *count += 1;
    }
  }
}
