use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct Rule {
  min: usize,
  max: usize,
  character: char,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let line_regex = Regex::new(r"(\d+)-(\d+) (.): (.+$)").unwrap();
  let count = reader.lines().map(Result::unwrap).filter(|line| {
    let captures = line_regex.captures(&line).unwrap();
    let rule = Rule {
      min: str::parse::<usize>(&captures[1]).unwrap(),
      max: str::parse::<usize>(&captures[2]).unwrap(),
      character: captures[3].chars().next().unwrap(),
    };

    let password = &captures[4];
    rule.check_password(password)
  }).count();

  println!("{} matched passwords", count);
  Ok(())
}

impl Rule {
  fn check_password(&self, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == self.character).count();
    count >= self.min && count <= self.max
  }
}
