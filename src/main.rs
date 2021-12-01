use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let mut depths = input.lines().map(|l| l.unwrap().parse::<u32>().unwrap());

  let mut window: VecDeque<u32> = VecDeque::with_capacity(3);
  for _ in 0..3 {
    window.push_back(depths.next().unwrap());
  }

  let mut count = 0;
  for depth in depths {
    let prev: u32 = window.iter().sum();
    window.pop_front();
    window.push_back(depth);
    let curr: u32 = window.iter().sum();

    if curr > prev {
      count += 1;
    }
  }

  println!("{}", count);

  Ok(())
}
