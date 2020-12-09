use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use circular_queue::CircularQueue;

struct Entry {
  number: i32,
  sum_cache: HashSet<i32>,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let numbers = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap());

  let window_size = 25;
  let mut window: CircularQueue<Entry> = CircularQueue::with_capacity(window_size);
  for number in numbers {
    for entry in window.iter_mut() {
      entry.sum_cache.insert(entry.number + number);
    }

    if window.is_full() {
      let mut result = false;
      for entry in window.iter() {
        result = result || entry.sum_cache.contains(&number);
      }

      if !result {
        println!("invalid entry {}", number);
        break;
      }
    }

    window.push(Entry { number: number, sum_cache: HashSet::new() });
  }

  Ok(())
}
