use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let max = reader.lines().map(|line| {
    line.unwrap().chars().fold((0, 0), |(row, col), c| {
      match c {
        'F' => (row << 1, col),
        'B' => ((row << 1) + 1, col),
        'L' => (row, col << 1),
        'R' => (row, (col << 1) + 1),
        _ => panic!(),
      }
    })
  }).map(|(row, col)| row * 8 + col).max().unwrap();

  println!("Maximum ID: {}", max);
  Ok(())
}
