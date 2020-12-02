use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct Rule {
  index_one: usize,
  index_two: usize,
  character: char,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let line_regex = Regex::new(r"(\d+)-(\d+) (.): (.+$)").unwrap();
  let count = reader.lines().map(Result::unwrap).filter(|line| {
    let captures = line_regex.captures(&line).unwrap();
    let rule = Rule {
      index_one: str::parse::<usize>(&captures[1]).unwrap() - 1,
      index_two: str::parse::<usize>(&captures[2]).unwrap() - 1,
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
    let characters = password.chars().collect::<Vec<char>>();
    let index_one_match = characters.get(self.index_one).map(|&character| character == self.character).unwrap_or(false);
    let index_two_match = characters.get(self.index_two).map(|&character| character == self.character).unwrap_or(false);
    index_one_match ^ index_two_match
  }
}
