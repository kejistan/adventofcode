use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::RangeInclusive;
use regex::Regex;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let rule_regex = Regex::new(r"(\d+)-(\d+) or (\d+)-(\d+)").unwrap();

  let mut rules = Vec::new();
  let mut lines = reader.lines();
  loop {
    match lines.next() {
      Some(Ok(next)) => {
        if next.is_empty() {
          break;
        }

        let captures = rule_regex.captures(&next).unwrap();
        let one = to_i32(&captures[1])..=to_i32(&captures[2]);
        let two = to_i32(&captures[3])..=to_i32(&captures[4]);
        rules.push((one, two));
      },
      _ => panic!(),
    }
  }

  lines.next();
  let _my_ticket = lines.next().unwrap().unwrap();
  lines.next();
  lines.next();

  let mut result = 0;
  for line in lines {
    result += line.unwrap().split(',').map(|s| s.parse::<i32>().unwrap()).filter(|num| !any_match(&rules, num)).sum::<i32>();
  }

  println!("{}", result);

  Ok(())
}

fn any_match(rules: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>, num: &i32) -> bool {
  for (one, two) in rules {
    if one.contains(num) || two.contains(num) {
      return true;
    }
  }
  false
}

fn to_i32(string: &str) -> i32 {
  string.parse::<i32>().unwrap()
}
