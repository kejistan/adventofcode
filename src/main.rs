use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut answers: HashSet<char> = HashSet::new();
  let mut total = 0;
  for l in reader.lines() {
    if let Ok(line) = l {
      if line.is_empty() {
        total += answers.len();
        answers.clear();
      } else {
        for c in line.chars() {
          answers.insert(c);
        }
      }
    } else {
      panic!()
    }
  }
  total += answers.len();

  println!("Total answers: {}", total);
  Ok(())
}
