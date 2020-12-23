use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
  let input = vec![6,2,4,3,9,7,1,5,8];
  let max = input.len() as u8;
  let mut circle = input.into_iter().collect::<VecDeque<u8>>();
  for _ in 0..100 {
    let current = *circle.front().unwrap();
    circle.rotate_left(1);

    let mut cups = Vec::with_capacity(3);
    cups.push(circle.pop_front().unwrap());
    cups.push(circle.pop_front().unwrap());
    cups.push(circle.pop_front().unwrap());

    let next = next(current, &cups, max);

    while *circle.back().unwrap() != next {
      circle.rotate_right(1);
    }

    for cup in cups.into_iter() {
      circle.push_back(cup);
    }

    while *circle.back().unwrap() != current {
      circle.rotate_left(1);
    }
  }

  while *circle.back().unwrap() != 1 {
    circle.rotate_left(1);
  }

  let result = circle.into_iter().take(max as usize - 1).map(|cup| format!("{}", cup)).collect::<String>();
  println!("{}", result);

  Ok(())
}

fn next(mut current: u8, cups: &[u8], max: u8) -> u8 {
  loop {
    if current == 1 {
      current = max;
    } else {
      current -= 1;
    }

    if !cups.contains(&current) {
      break;
    }
  }

  current
}
