use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut adapters = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
  adapters.sort();

  let mut threes = 1;
  let mut ones = 0;
  let mut prev = 0;

  for adapter in adapters {
    match adapter - prev {
      1 => ones += 1,
      3 => threes += 1,
      _ => (),
    }

    prev = adapter;
  }

  println!("{} * {} = {}", ones, threes, ones * threes);

  Ok(())
}
