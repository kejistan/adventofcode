use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let depths = input.lines().map(|l| l.unwrap().parse::<u32>().unwrap());

  let (count, _) = depths.fold((0, None), |(count, prev), curr| {
    if let Some(prev) = prev {
      if curr > prev {
        (count + 1, Some(curr))
      } else {
        (count, Some(curr))
      }
    } else {
      (count, Some(curr))
    }
  });

  println!("{}", count);

  Ok(())
}
