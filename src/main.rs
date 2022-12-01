use std::io;
use std::io::BufReader;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let max = input.groups().map(|lines| -> u32 {
    lines.unwrap().lines().map(|line| -> u32 {
      line.parse::<u32>().unwrap()
    }).sum()
  }).max();

  println!("{}", max.unwrap_or(0));

  Ok(())
}