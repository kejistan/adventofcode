use std::io;
use std::io::prelude::*;
use std::io::BufReader;

enum Operation {
  NOP,
  ACC,
  JMP,
}

struct Instruction {
  op: Operation,
  operand: isize,
  visited: bool,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut instructions = reader.lines().map(|l| {
    let line = l.unwrap();
    let operation = match &line[0..3] {
      "nop" => Operation::NOP,
      "acc" => Operation::ACC,
      "jmp" => Operation::JMP,
      _ => panic!(),
    };
    let negative = &line[4..5] == "-";
    let mut operand = line[5..].parse::<isize>().unwrap();

    if negative {
      operand *= -1;
    }

    Instruction {
      op: operation,
      operand: operand,
      visited: false,
    }
  }).collect::<Vec<Instruction>>();

  let mut accumulator = 0;
  let mut pc = 0;

  loop {
    let instruction = instructions.get_mut(pc).unwrap();
    if instruction.visited {
      break;
    }

    instruction.visited = true;

    match instruction.op {
      Operation::NOP => pc += 1,
      Operation::ACC => {
        accumulator += instruction.operand;
        pc += 1;
      },
      Operation::JMP => pc = ((pc as isize) + instruction.operand) as usize,
    }
  }

  println!("accumulator: {}", accumulator);

  Ok(())
}
