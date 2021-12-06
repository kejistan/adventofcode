use std::io;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
  let re = Regex::new(r"\d+").unwrap();
  let input = BufReader::new(io::stdin());

  let mut fish_counts = VecDeque::with_capacity(9);
  for _ in 0..9 {
    fish_counts.push_back(0);
  }
  for age in re.captures_iter(&input.lines().next().unwrap().unwrap()).map(|caps| caps.get(0).unwrap().as_str().parse::<usize>().unwrap()) {
    fish_counts[age] += 1;
  }

  for _ in 0..256 {
    advance(&mut fish_counts);
  }

  println!("{}", fish_counts.iter().sum::<u64>());

  Ok(())
}

fn advance(fish_counts: &mut VecDeque<u64>) {
  let fish = fish_counts.pop_front().unwrap();
  fish_counts[6] += fish;
  fish_counts.push_back(fish);
}
