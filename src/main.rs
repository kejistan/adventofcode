use std::collections::HashSet;
use std::{io};
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let contents = input.lines().map(|line| {
    let line_string = line.unwrap();
    line_string.bytes().map(byte_to_priority).collect::<HashSet<u8>>()
  });

  let mut count = 1;
  let mut contents_iter = contents.into_iter();
  let mut shared_items = contents_iter.next().unwrap();
  let mut score = 0;
  for content in contents_iter {
    if count == 0 {
      score += shared_items.into_iter().next().unwrap() as u32;
      shared_items = content;
    } else {
      shared_items = shared_items.intersection(&content).map(|&a| a).collect();
    }

    count = (count + 1) % 3;
  }
  score += shared_items.into_iter().next().unwrap() as u32;

  println!("{}", score);

  Ok(())
}

fn byte_to_priority(byte: u8) -> u8 {
  if byte >= 'a' as u8 {
    byte - 'a' as u8 + 1
  } else {
    byte - 'A' as u8 + 27
  }
}
