use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::RangeInclusive;
use regex::Regex;

#[derive(Eq, PartialEq, Hash)]
struct Rule {
  is_departure: bool,
  ranges: Vec<RangeInclusive<u32>>,
}

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
        let one = to_u32(&captures[1])..=to_u32(&captures[2]);
        let two = to_u32(&captures[3])..=to_u32(&captures[4]);
        rules.push(Rule {
          is_departure: next.starts_with("departure"),
          ranges: vec![one, two],
        });
      },
      _ => panic!(),
    }
  }

  lines.next();
  let my_ticket = parse_ticket(&lines.next().unwrap().unwrap());
  lines.next();
  lines.next();

  let mut columns: Vec<Vec<u32>> = Vec::with_capacity(my_ticket.len());
  for _ in 0..my_ticket.len() {
    columns.push(vec![]);
  }

  let valid_tickets = lines.map(|line| parse_ticket(&line.unwrap()))
    .filter(|ticket| !ticket.iter().any(|num| !any_match(&rules, num)));
  for ticket in valid_tickets {
    for (i, num) in ticket.into_iter().enumerate() {
      columns[i].push(num)
    }
  }

  let departure_indices = rules_for_columns(&rules, &columns).into_iter()
    .enumerate()
    .filter(|(_, rule)| rule.is_departure)
    .map(|(i, _)| i);

  let result = departure_indices.fold(1, |result, i| {
    result * my_ticket[i] as u64
  });

  println!("{}", result);

  Ok(())
}

fn rules_for_columns<'a>(rules: &'a Vec<Rule>, columns: &Vec<Vec<u32>>) -> Vec<&'a Rule> {
  let mut valid_rules_for_columns = columns.into_iter()
    .map(|values| find_valid_rules(&rules, &values))
    .collect::<Vec<HashSet<&Rule>>>();

  loop {
    if valid_rules_for_columns.iter().all(|set| set.len() == 1) {
      break;
    }

    let locked_rules = valid_rules_for_columns.iter()
      .filter(|set| set.len() == 1)
      .map(|set| *set.iter().next().unwrap())
      .collect::<Vec<&Rule>>();
    
    for set in valid_rules_for_columns.iter_mut().filter(|set| set.len() != 1) {
      for rule in locked_rules.iter() {
        set.remove(rule);
      }
    }
  }

  valid_rules_for_columns.into_iter()
    .map(|set| *set.iter().next().unwrap())
    .collect::<Vec<&Rule>>()
}

fn find_valid_rules<'a>(rules: &'a Vec<Rule>, numbers: &Vec<u32>) -> HashSet<&'a Rule> {
  rules.iter().filter(|rule| {
    numbers.iter().all(|num| rule.is_valid(num))
  }).collect::<HashSet<&Rule>>()
}

fn parse_ticket(line: &str) -> Vec<u32> {
  line.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

fn any_match(rules: &Vec<Rule>, num: &u32) -> bool {
  rules.iter().any(|rule| rule.is_valid(num))
}

fn to_u32(string: &str) -> u32 {
  string.parse::<u32>().unwrap()
}

impl Rule {
  fn is_valid(&self, num: &u32) -> bool {
    self.ranges.iter().any(|range| range.contains(num))
  }
}
