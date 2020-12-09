use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let numbers = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap());

  let target = 41682220;
  let mut window: VecDeque<i32> = VecDeque::new();
  for number in numbers {
    window.push_back(number);
    let mut sum: i32 = window.iter().sum();
    
    while sum > target {
      if let Some(popped) = window.pop_front() {
        sum -= popped;
      }
    }

    if sum == target {
      let min = window.iter().min().unwrap();
      let max = window.iter().max().unwrap();
      println!("{} + {} = {}", min, max, min + max);
      break;
    }
  }

  Ok(())
}
