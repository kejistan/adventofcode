use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

enum Instruction {
  Mask(String),
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
      Instruction::Mask(captures[1].to_string())
    } else {
      panic!();
    }
  });

  let mut memory = HashMap::new();
  let mut masks = Vec::new();
  masks.push((0, u64::MAX));

  for instruction in instructions {
    match instruction {
      Instruction::Mask(string) => {
        masks = masks_to_write(&string);
      },
      Instruction::Mem(address, num) => {
        let addresses = masks.iter().map(|(or, and)| (address | or) & and);
        for addr in addresses {
          memory.insert(addr, num);
        }
      }
    }
  }

  let result: u64 = memory.values().sum();

  println!("{}", result);

  Ok(())
}

fn masks_to_write(mask: &str) -> Vec<(u64, u64)> {
  let mut masks = Vec::new();
  masks.push((0, u64::MAX));

  for character in mask.chars() {
    match character {
      '0' => {
        for (or, and) in masks.iter_mut() {
          *or = *or << 1;
          *and = (*and << 1) + 1;
        }
      }
      '1' => {
        for (or, and) in masks.iter_mut() {
          *or = (*or << 1) + 1;
          *and = (*and << 1) + 1;
        }
      }
      'X' => {
        for (or, and) in masks.iter_mut() {
          *or = (*or << 1) + 1;
          *and = (*and << 1) + 1;
        }
        let mut alternates = Vec::with_capacity(masks.len());
        for (or, and) in masks.iter() {
          alternates.push((*or & (u64::MAX - 1), and & (u64::MAX - 1)));
        }

        masks.append(&mut alternates);
      }
      _ => panic!(),
    }
  }

  masks
}
