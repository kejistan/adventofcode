use std::{io};
use std::io::{BufReader, BufRead};
use regex::{Regex};

struct Range {
  start: u8,
  end: u8,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let r = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

  let count: u32 = input.lines().map(|line| {
    let string = line.unwrap();
    let captures = r.captures(&string).unwrap();
    let one = Range {
      start: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
      end: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
    };
    let two = Range {
      start: captures.get(3).unwrap().as_str().parse::<u8>().unwrap(),
      end: captures.get(4).unwrap().as_str().parse::<u8>().unwrap(),
    };

    if one.overlaps(&two) {
      1
    } else {
      0
    }
  }).sum();

  println!("{}", count);

  Ok(())
}

impl Range {
  fn overlaps(&self, other: &Range) -> bool {
    !(other.end < self.start || other.start > self.end)
  }
}
