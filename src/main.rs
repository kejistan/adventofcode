use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct Food {
  ingredients: Vec<String>,
  allergens: Vec<String>,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let allergen_regex = Regex::new(r"(\w+)[,)]").unwrap();
  let foods = reader.lines().map(|l| {
    let line = l.unwrap();
    let parts = line.split("(contains ").collect::<Vec<&str>>();
    let ingredients = parts[0].split(' ').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>();
    let allergens = allergen_regex.captures_iter(parts[1]).map(|cap| cap[1].to_string()).collect::<Vec<String>>();

    Food {
      ingredients,
      allergens,
    }
  });

  let mut allergen_possibilities: HashMap<String, HashSet<String>> = HashMap::new();
  for food in foods {
    let possibilities = food.ingredients.iter().map(String::clone).collect::<HashSet<String>>();
    for allergen in food.allergens {
      if let Some(set) = allergen_possibilities.get_mut(allergen.as_str()) {
        *set = set.intersection(&possibilities).map(|s| s.clone()).collect::<HashSet<String>>();
      } else {
        allergen_possibilities.insert(allergen, possibilities.clone());
      }
    }
  }

  let mut known_allergens = Vec::with_capacity(allergen_possibilities.len());
  while !allergen_possibilities.is_empty() {
    let (a, _) = allergen_possibilities.iter().find(|(_, set)| set.len() == 1).unwrap();
    let allergen = a.clone();
    let (allergen, set) = allergen_possibilities.remove_entry(&allergen).unwrap();
    let ingredient = set.into_iter().next().unwrap();
    for (_, set) in allergen_possibilities.iter_mut() {
      set.remove(&ingredient);
    }
    known_allergens.push((allergen, ingredient));
  }

  known_allergens.sort_by(|(a, _), (b, _)| a.cmp(b));
  let sorted_ingredients = known_allergens.into_iter().map(|(_, ingredient)| ingredient).collect::<Vec<String>>();
  println!("{}", sorted_ingredients.join(","));

  Ok(())
}
