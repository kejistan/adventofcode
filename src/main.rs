use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let numbers = reader.lines().map(Result::unwrap).map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let result = numbers.iter().cartesian_product(numbers.iter()).find(|(&a, &b)| {
    a + b == 2020
  });
  match result {
    Some((a, b)) => println!("{} * {} = {}", a, b, a * b),
    None => println!("no result found"),
  }
  Ok(())
}
