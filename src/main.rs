use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut lines = reader.lines();
  let minimum = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
  let buses = lines.next().unwrap().unwrap().split_terminator(',')
    .filter(|bus| *bus != "x")
    .map(|bus| bus.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let mut target = minimum;
  loop {
    for bus in buses.iter() {
      if target % bus == 0 {
        println!("bus {} at {}", bus, target);
        let wait = target - minimum;
        println!("wait {}", wait);
        println!("result: {}", wait * bus);
        return Ok(());
      }
    }
    target += 1;
  }
}
