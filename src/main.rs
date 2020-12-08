use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Operation {
  NOP,
  ACC,
  JMP,
}

struct Instruction {
  op: Operation,
  operand: isize,
  accumulator: Option<isize>,
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
      accumulator: None,
      visited: false,
    }
  }).collect::<Vec<Instruction>>();

  let first_instruction = &mut instructions[0];
  first_instruction.accumulator = Some(0);
  first_instruction.visited = true;

  // initialize cached accumulators
  run_instructions(&mut instructions, 0, true);

  let mut target = 0;
  loop {
    if target >= instructions.len() {
      panic!("fix not found!");
    }

    toggle_instruction_at(&mut instructions, target);

    if let Some(result) = run_instructions(&mut instructions, target, false) {
      println!("result: {}", result);
      break;
    }

    target += 1;
    toggle_instruction_at(&mut instructions, target - 1);
  }

  Ok(())
}

fn run_instructions(instructions: &mut Vec<Instruction>, initial_pc: usize, set_accumulators: bool) -> Option<isize> {
  // special case for first instruction, we want to ignore the accumulator cache's
  // presence.
  let mut pc = initial_pc;
  let first_instruction = instructions.get(pc).unwrap();
  if first_instruction.accumulator == None {
    // this is pc is unreachable code
    return None;
  }

  let mut accumulator = first_instruction.accumulator.unwrap();
  if first_instruction.op == Operation::ACC {
    accumulator += first_instruction.operand;
  }
  pc = calculate_pc(first_instruction, pc);

  loop {
    match instructions.get_mut(pc) {
      None => break,
      Some(instruction) => {
        if instruction.visited == true {
          return None;
        }

        instruction.visited = true;
        if set_accumulators {
          instruction.accumulator = Some(accumulator);
        }

        pc = calculate_pc(instruction, pc);
        if instruction.op == Operation::ACC {
          accumulator += instruction.operand;
        }
      },
    }
  }

  if pc == instructions.len() {
    Some(accumulator)
  } else {
    None
  }
}

fn calculate_pc(instruction: &Instruction, pc: usize) -> usize {
  match instruction.op {
    Operation::NOP => pc + 1,
    Operation::ACC => pc + 1,
    Operation::JMP => {
      let new_pc = (pc as isize) + instruction.operand;
      if new_pc < 0 {
        panic!("negative pc");
      } else {
        new_pc as usize
      }
    }
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
