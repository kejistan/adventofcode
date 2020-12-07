use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let base_rule_regex = Regex::new(r"^(\w+ \w+) bags contain ").unwrap();
  let empty_content_regex = Regex::new(r"no other bags.").unwrap();
  let content_regex = Regex::new(r"\d+ (\w+ \w+) bags?").unwrap();

  let parsed_rules = reader.lines().map(|l| {
    let line = l.unwrap();
    let bag_type: String = base_rule_regex.captures(&line).unwrap().get(1).map_or("", |s| s.as_str()).to_string();
    let content_str: &str = &line[(bag_type.len() + " bags contain ".len())..];
    if empty_content_regex.is_match(content_str) {
      (bag_type, HashSet::default())
    } else {
      let content = content_regex.captures_iter(content_str).map(|cap| cap.get(1).unwrap().as_str().to_string());
      (bag_type, content.collect::<HashSet<String>>())
    }
  });
  
  // Reverse map of each bag -> bags that contain it
  let mut reverse_rules: HashMap<String, HashSet<String>> = HashMap::new();
  for (bag_type, contents) in parsed_rules {
    for content in contents {
      if let Some(containers) = reverse_rules.get_mut(&content) {
        containers.insert(bag_type.clone());
      } else {
        let mut containers = HashSet::new();
        containers.insert(bag_type.clone());
        reverse_rules.insert(content, containers);
      }
    }
  }

  let mut shiny_gold_containers = HashSet::new();
  for container in reverse_rules.get("shiny gold").unwrap() {
    recursive_insert_containers(&mut shiny_gold_containers, &reverse_rules, container.clone());
  }

  println!("shiny gold bag contained by {} types", shiny_gold_containers.len());

  Ok(())
}

fn recursive_insert_containers(results: &mut HashSet<String>, reverse_rules: &HashMap<String, HashSet<String>>, bag: String) {
  let containers = reverse_rules.get(&bag);
  if results.insert(bag) {
    // No containers in reverse_rules means the bag has no possible parents
    if let Some(containers) = containers {
      for container in containers {
        recursive_insert_containers(results, reverse_rules, container.clone());
      }
    }
  }
}
