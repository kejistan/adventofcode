use std::{io};
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let line = input.lines().next().unwrap()?;

  let (result, _) = line.as_bytes().windows(4).enumerate().find(|(i, window)| {
    for i in 0..window.len() {
      let element = window[i];
      if window[i+1..].contains(&element) {
        return false;
      }
    }

    true
  }).unwrap();

  println!("{}", result + 4);

  Ok(())
}
