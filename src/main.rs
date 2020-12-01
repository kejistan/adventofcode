use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let numbers = reader.lines().map(Result::unwrap).map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let result = numbers.iter().cartesian_product(numbers.iter()).cartesian_product(numbers.iter()).find(|((&a, &b), &c)| {
    a + b + c == 2020
  });
  match result {
    Some(((a, b), c)) => println!("{} * {} * {} = {}", a, b, c, a * b * c),
    None => println!("no result found"),
  }
  Ok(())
}
