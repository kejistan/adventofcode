use std::collections::HashSet;
use std::{io};
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let score: u32 = input.lines().map(|line| {
    let line_string = line.unwrap();
    let line_bytes = line_string.as_bytes();
    let line_len = line_bytes.len();

    let first_compartment = line_bytes[0..line_len / 2].iter().map(byte_to_priority).collect::<HashSet<u8>>();
    let second_compartment = line_bytes[line_len / 2..line_len].iter().map(byte_to_priority).collect::<HashSet<u8>>();

    let overlap = first_compartment.intersection(&second_compartment);
    *overlap.into_iter().next().unwrap() as u32
  }).sum();

  println!("{}", score);

  Ok(())
}

fn byte_to_priority(&byte: &u8) -> u8 {
  if byte >= 'a' as u8 {
    byte - 'a' as u8 + 1
  } else {
    byte - 'A' as u8 + 27
  }
}
