use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut adapters = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
  adapters.sort();
  adapters.push(adapters.last().unwrap() + 3);

  let mut window: VecDeque<(u64, i32)> = VecDeque::with_capacity(3);
  window.push_back((1, 0));
  for num in adapters {
    let mut count = 0;
    for (prev_count, prev) in window.iter() {
      if num - prev <= 3 {
        count += prev_count;
      }
    }

    if window.len() == 3 {
      window.pop_front();
    }
    window.push_back((count, num));
  }

  let (count, _) = window.back().unwrap();

  println!("{}", count);

  Ok(())
}
