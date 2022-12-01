use std::{io, mem};
use std::io::BufReader;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let top_three: u32 = input.groups().map(|lines| -> u32 {
    lines.unwrap().lines().map(|line| -> u32 {
      line.parse::<u32>().unwrap()
    }).sum()
  }).max_n(3).iter().sum();

  println!("{}", top_three);

  Ok(())
}

trait MaxN<T> {
  fn max_n(&mut self, num: usize) -> Vec<T>;
}

impl<T: Ord, U: Iterator<Item = T>> MaxN<T> for U {
  fn max_n(&mut self, num: usize) -> Vec<T> {
    self.fold(vec![], |mut max, mut value| {
      if max.len() < num {
        max.push(value);
        max.sort();
        max.reverse();
        return max;
      }

      for max_value in max.iter_mut() {
        if *max_value < value {
          mem::swap(&mut *max_value, &mut value);
        }
      }

      max
    })
  }
}