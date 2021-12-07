use std::io;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() -> io::Result<()> {
  let re = Regex::new(r"\d+").unwrap();
  let input = BufReader::new(io::stdin());

  let positions = re.captures_iter(&input.lines().next().unwrap().unwrap()).map(|caps| caps.get(0).unwrap().as_str().parse::<u32>().unwrap()).collect::<Vec<u32>>();
  let max = positions.iter().max().unwrap();

  let mut min = None;
  for i in 0..*max {
    let cost = total_fuel_cost(&positions, i);
    match min {
      None => min = Some(cost),
      Some(best_so_far) if best_so_far > cost => min = Some(cost),
      _ => {},
    }
  }

  println!("{}", min.unwrap());

  Ok(())
}

fn total_fuel_cost(positions: &Vec<u32>, target: u32) -> u32 {
  positions.iter().fold(0, |cost, pos| cost + (*pos as i32 - target as i32).abs() as u32)
}
