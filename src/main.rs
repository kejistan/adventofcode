use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

enum Command {
  Forward(u32),
  Up(u32),
  Down(u32),
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let re = Regex::new(r"^(?P<direction>[[:alpha:]]+) (?P<count>\d+)$").unwrap();

  let commands = input.lines().map(|l| {
    let line = l.unwrap();
    let cap = re.captures(&line).unwrap();
    let count = cap.name("count").unwrap().as_str().parse::<u32>().unwrap();

    match cap.name("direction").unwrap().as_str() {
      "forward" => Command::Forward(count),
      "up" => Command::Up(count),
      "down" => Command::Down(count),
      _ => panic!(),
    }
  });

  let mut pos = (0, 0);

  for command in commands {
    match command {
      Command::Forward(count) => pos.0 += count,
      Command::Down(count) => pos.1 += count,
      Command::Up(count) => pos.1 -= count,
    }
  }

  println!("{:?}", pos);
  println!("{}", pos.0 * pos.1);

  Ok(())
}
