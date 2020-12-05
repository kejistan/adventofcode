use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut found_seats = reader.lines().map(|line| {
    let string = line.unwrap();
    let result = string.chars().fold((0, 0), |(row, col), c| {
      match c {
        'F' => (row << 1, col),
        'B' => ((row << 1) + 1, col),
        'L' => (row, col << 1),
        'R' => (row, (col << 1) + 1),
        _ => panic!(),
      }
    });

    println!("{} = ({}, {})", string, result.0, result.1);
    result
  }).map(|(row, col)| row * 8 + col).collect::<Vec<i32>>();
  
  found_seats.sort();

  let (_, available_seat) = found_seats.into_iter().fold((None, None), |acc, seat| {
    match acc {
      (None, _) => (Some(seat), None),
      (Some(prev), None) => {
        if prev + 1 != seat {
          (Some(seat), Some(prev + 1))
        } else {
          (Some(seat), None)
        }
      },
      _ => acc,
    }
  });

  println!("Available seat: {}", available_seat.unwrap());
  Ok(())
}
