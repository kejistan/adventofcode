use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let mut width = 0;
  let mut numbers = vec![];

  for l in input.lines() {
    let mut number = 0;
    let line = l.unwrap();
    if width == 0 {
      width = line.len();
    }

    for bit in line.chars() {
      number <<= 1;

      match bit {
        '1' => {
          number += 1;
        }
        _ => {}
      }
    }

    numbers.push(number);
  }

  let (gamma, epsilon) = find_rates(&numbers, width);

  println!("{:0width$b}", gamma, width = width);
  println!("{:0width$b}", epsilon, width = width);

  let oxygen = find_rating(numbers.clone(), width, Type::Gamma);
  let co2 = find_rating(numbers.clone(), width, Type::Epsilon);
  println!("oxygen {}", oxygen);
  println!("co2 {}", co2);
  println!("{}", oxygen * co2);

  Ok(())
}

enum Type {
  Gamma,
  Epsilon,
}

fn find_rating(mut numbers: Vec<u32>, width: usize, rate_type: Type) -> u32 {
  let mut bit = 1;
  while width >= bit && numbers.len() > 1 {
    let (gamma, epsilon) = find_rates(&numbers, width);
    let rate = match rate_type {
      Type::Gamma => gamma,
      Type::Epsilon => epsilon,
    };

    let mask = 1 << width - bit;
    numbers = numbers.into_iter().filter(|num| num & mask == rate & mask).collect::<Vec<u32>>();

    bit += 1;
  }

  numbers[0]
}

fn find_rates(numbers: &Vec<u32>, width: usize) -> (u32, u32) {
  let mut one_bits = vec![];
  for num in numbers {
    for i in 0..width {
      if one_bits.len() == i {
        one_bits.push(0);
      }

      let mask = 1 << i;
      if num & mask != 0 {
        one_bits[i] += 1;
      }
    }
  }

  let mut gamma = 0;
  let mut epsilon = 0;

  one_bits.reverse();

  for count in one_bits {
    gamma <<= 1;
    epsilon <<= 1;

    if 2 * count >= numbers.len() {
      gamma += 1;
    } else {
      epsilon += 1;
    }
  }
  
  (gamma, epsilon)
}
