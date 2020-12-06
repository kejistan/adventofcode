use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut answers: HashMap<char, i32> = HashMap::new();
  let mut total = 0;
  let mut group_size = 0;
  for l in reader.lines() {
    if let Ok(line) = l {
      if line.is_empty() {
        total += answers.iter().filter(|(_, &count)| count == group_size).count();
        answers.clear();
        group_size = 0;
      } else {
        group_size += 1;
        for c in line.chars() {
          if let Some(count) = answers.get_mut(&c) {
            *count += 1;
          } else {
            answers.insert(c, 1);
          }
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
