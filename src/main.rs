use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

#[derive(Debug)]
enum Rule {
  Literal(char),
  Ref(u32),
  Or(Vec<Rule>),
  Seq(Vec<Rule>),
}

use Rule::*;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let mut lines = reader.lines().into_iter();
  let rule_line_regex = Regex::new(r"^(\d+): (.+)$").unwrap();
  
  let mut tokenized_rules = HashMap::new();
  while let Some(l) = lines.next() {
    let line = l.unwrap();
    if line.is_empty() {
      break;
    }

    let captures = rule_line_regex.captures(&line).unwrap();
    let label = captures[1].parse::<u32>().unwrap();
    let rule = &captures[2];

    if rule.chars().nth(0).unwrap() == '"' {
      tokenized_rules.insert(label, Literal(rule.chars().nth(1).unwrap()));
    } else if rule.contains('|') {
      tokenized_rules.insert(label, Or(rule.split('|').map(parse_seq).collect()));
    } else {
      tokenized_rules.insert(label, parse_seq(rule));
    }
  }

  tokenized_rules.remove(&0);
  tokenized_rules.remove(&8);
  tokenized_rules.remove(&11);

  let mut count = 0;
  for l in lines {
    let line = l.unwrap();
    let result = check_rules(&tokenized_rules, &line);

    if result {
      count += 1;
    }
  }

  println!("{}", count);

  Ok(())
}

fn check_rules(tokenized_rules: &HashMap<u32, Rule>, mut string: &str) -> bool {
  // rule 0: 8 11
  // rule 8: 42 | 42 8
  // rule 11: 42 31 | 42 11 31

  let rule42 = tokenized_rules.get(&42).unwrap();
  while !string.is_empty() {
    if let (true, more) = check_rule(tokenized_rules, rule42, string) {
      string = more;
      if check_rule_11(tokenized_rules, string) {
        return true;
      }
    } else {
      break;
    }
  }

  false
}

fn check_rule_11(tokenized_rules: &HashMap<u32, Rule>, mut string: &str) -> bool {
  let rule42 = tokenized_rules.get(&42).unwrap();
  let rule31 = tokenized_rules.get(&31).unwrap();

  let mut num = 0;
  while !string.is_empty() {
    if let (true, more) = check_rule(tokenized_rules, rule42, string) {
      num += 1;
      string = more;

      let mut cont = string;
      let mut i = 0;
      while i < num && !cont.is_empty() {
        if let (true, more) = check_rule(tokenized_rules, rule31, cont) {
          cont = more;
        } else {
          break;
        }

        i += 1;
      }

      if i == num && cont.is_empty() {
        return true;
      }
    } else {
      break;
    }
  }

  false
}

fn parse_seq(string: &str) -> Rule {
  Rule::Seq(string.split(' ').filter(|s| !s.is_empty()).map(str::parse).map(Result::unwrap).map(Ref).collect::<Vec<Rule>>())
}

fn check_rule<'a>(tokenized_rules: &HashMap<u32, Rule>, rule: &Rule, string: &'a str) -> (bool, &'a str) {
  if string.is_empty() {
    return (false, string);
  }

  match rule {
    Literal(character) => {
      let is_match = string.chars().nth(0).unwrap() == *character;
      (is_match, &string[1..])
    },
    Ref(label) => {
      check_rule(tokenized_rules, tokenized_rules.get(label).unwrap(), string)
    },
    Seq(rules) => {
      let mut is_match = true;
      let mut more_str = string;
      for rule in rules.iter() {
        let (did_match, more) = check_rule(tokenized_rules, rule, more_str);
        is_match &= did_match;
        more_str = more;
        if !is_match {
          break;
        }
      }

      (is_match, more_str)
    },
    Or(rules) => {
      for rule in rules.iter() {
        let (did_match, more) = check_rule(tokenized_rules, rule, string);
        if did_match {
          return (true, more);
        }
      }

      (false, string)
    }
  }
}
