use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
  let mut numbers = HashMap::new();

  let seed = vec![9,3,1,0,8,4];
  let mut turn_number = 1;
  for num in seed {
    numbers.insert(num, turn_number);
    turn_number += 1;
  }

  let mut next = 0;
  while turn_number < 30000000 {
    if let Some(prev) = numbers.insert(next, turn_number) {
      next = turn_number - prev;
    } else {
      next = 0;
    }

    turn_number += 1;
  }

  println!("{}", next);

  Ok(())
}
