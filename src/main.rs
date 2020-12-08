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

  let mut target = 0;
  loop {
    if target >= instructions.len() {
      panic!("fix not found!");
    }

    toggle_instruction_at(&mut instructions, target);
    target += 1;

    // reset instruction state
    for instruction in instructions.iter_mut() {
      instruction.visited = false;
    }

    let (success, accumulator) = run_instructions(&mut instructions);
    if success {
      println!("accumulator: {}", accumulator);
      break;
    }

    toggle_instruction_at(&mut instructions, target - 1);
  }

  Ok(())
}

fn run_instructions(instructions: &mut Vec<Instruction>) -> (bool, isize) {
  let mut pc = 0;
  let mut accumulator = 0;
  loop {
    match instructions.get_mut(pc) {
      None => break,
      Some(instruction) => {
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
          Operation::JMP => {
            let new_pc = (pc as isize) + instruction.operand;
            if new_pc < 0 {
              break;
            } else {
              pc = new_pc as usize;
            }
          }
        }
      },
    }
  }

  if pc == instructions.len() {
    (true, accumulator)
  } else {
    (false, accumulator)
  }
}

fn toggle_instruction_at(instructions: &mut Vec<Instruction>, target: usize) {
  let instruction = match instructions.get_mut(target) {
    None => panic!("None for target {}", target),
    Some(i) => i,
  };

  match instruction.op {
    Operation::ACC => (),
    Operation::JMP => {
      instruction.op = Operation::NOP
    },
    Operation::NOP => {
      instruction.op = Operation::JMP
    },
  }
}
