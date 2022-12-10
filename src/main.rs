use std::{io};
use std::io::{BufReader, BufRead};

enum Instruction {
  Noop,
  Addx(i8),
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let instructions = input.lines().map(|result| {
    let line = result.unwrap();
    if line == "noop" {
      return Instruction::Noop;
    }

    let (_command, number) = line.split_at(5);
    Instruction::Addx(number.parse::<i8>().unwrap())
  });

  let mut x: i64 = 1;
  let cycles = instructions.flat_map(|instruction| {
    match instruction {
      Instruction::Noop => vec![x].into_iter(),
      Instruction::Addx(arg) => {
        let next_x = x + arg as i64;
        let ret = vec![x, next_x].into_iter();
        x = next_x;
        return ret;
      },
    }
  }).enumerate().map(|(cycle, x)| (cycle + 2, x));

  let result: i64 = cycles
    .filter(|(cycle, _)| *cycle == 20 || (*cycle > 20 && (cycle - 20) % 40 == 0))
    .map(|(cycle, x)| -> i64 { cycle as i64 * x as i64 }).sum();
  println!("{}", result);

  Ok(())
}
