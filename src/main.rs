use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let (_, count) = reader.lines().map(Result::unwrap).map(|line| {
    line.chars().map(|c| c != '.').collect::<Vec<bool>>()
  }).fold((0, 0), |(x, count), line| {
    let next = x + 3;
    if line[x % line.len()] {
      (next, count + 1)
    } else {
      (next, count)
    }
  });

  println!("{} trees on slope (1, 3)", count);
  Ok(())
}
