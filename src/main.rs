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

  let mut ingredient_counts: HashMap<String, u32> = HashMap::new();
  let mut allergen_possibilities: HashMap<String, HashSet<String>> = HashMap::new();
  for food in foods {
    let possibilities = food.ingredients.iter().map(String::clone).collect::<HashSet<String>>();
    for ingredient in food.ingredients.iter() {
      if let Some(count) = ingredient_counts.get_mut(ingredient) {
        *count += 1;
      } else {
        ingredient_counts.insert(ingredient.clone(), 1);
      }
    }
    for allergen in food.allergens {
      if let Some(set) = allergen_possibilities.get_mut(allergen.as_str()) {
        *set = set.intersection(&possibilities).map(|s| s.clone()).collect::<HashSet<String>>();
      } else {
        allergen_possibilities.insert(allergen, possibilities.clone());
      }
    }
  }

  let possibilities = allergen_possibilities.values().fold(HashSet::default(), |result, possibilities| {
    possibilities.union(&result).map(String::clone).collect::<HashSet<String>>()
  });

  let count: u32 = ingredient_counts.keys().map(String::clone).collect::<HashSet<String>>().difference(&possibilities).map(|key| ingredient_counts.get(key).unwrap()).sum();
  println!("{}", count);

  Ok(())
}
