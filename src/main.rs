use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

enum Instruction {
  Mask(u64, u64),
  Mem(u64, u64),
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
  let mask_regex = Regex::new(r"mask = ([01X]+)").unwrap();

  let instructions = reader.lines().map(|l| {
    let line = l.unwrap();
    if let Some(captures) = mem_regex.captures(line.as_str()) {
      Instruction::Mem(captures[1].parse::<u64>().unwrap(), captures[2].parse::<u64>().unwrap())
    } else if let Some(captures) = mask_regex.captures(line.as_str()) {
      let (or, and) = captures[1].chars().fold((0, u64::MAX), |(or, and), bit| {
        match bit {
          'X' => (or << 1, (and << 1) + 1),
          '0' => (or << 1, and << 1),
          '1' => ((or << 1) + 1, (and << 1) + 1),
          _ => panic!(),
        }
      });
      Instruction::Mask(or, and)
    } else {
      panic!();
    }
  });

  let mut memory = HashMap::new();
  let mut mask = (0, u64::MAX);

  for instruction in instructions {
    match instruction {
      Instruction::Mask(or, and) => mask = (or, and),
      Instruction::Mem(address, num) => {
        let actual = (num | mask.0) & mask.1;
        memory.insert(address, actual);
      }
    }
  }

  let result: u64 = memory.values().sum();

  println!("{}", result);

  Ok(())
}
