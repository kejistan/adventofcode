use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let base_rule_regex = Regex::new(r"^(\w+ \w+) bags contain ").unwrap();
  let empty_content_regex = Regex::new(r"no other bags.").unwrap();
  let content_regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

  let parsed_rules = reader.lines().map(|l| {
    let line = l.unwrap();
    let bag_type: String = base_rule_regex.captures(&line).unwrap().get(1).map_or("", |s| s.as_str()).to_string();
    let content_str: &str = &line[(bag_type.len() + " bags contain ".len())..];
    if empty_content_regex.is_match(content_str) {
      (bag_type, HashMap::default())
    } else {
      let mut counts = HashMap::new();
      for capture in content_regex.captures_iter(content_str) {
        let count = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let bag_type = capture.get(2).unwrap().as_str().to_string();

        counts.insert(bag_type, count);
      }
      (bag_type, counts)
    }
  });

  // Mapping of each bag and the bags it contains
  let mut rules = HashMap::new();
  for (bag_type, contents) in parsed_rules {
    rules.insert(bag_type, contents);
  }

  let result = recursive_count_children(&rules, "shiny gold");

  println!("shiny gold bag contains {} other bags", result);

  Ok(())
}

fn recursive_count_children(rules: &HashMap<String, HashMap<String, i32>>, bag: &str) -> i32 {
  rules.get(bag).unwrap().iter().fold(0, |total, (bag, count)| {
    total + count * (1 + recursive_count_children(rules, bag))
  })
}
