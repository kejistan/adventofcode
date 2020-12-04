use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct Field
{
  validation: fn(&str) -> bool,
  valid: bool,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut normalized_passport_info: Vec<String> = Vec::new();
  let mut valid_count = 0;
  for line in reader.lines() {
    let string = line.unwrap();
    if string.is_empty() {
      if validate_passport(&normalized_passport_info) {
        valid_count += 1;
      }
      normalized_passport_info.clear();
    }

    for string in string.split_whitespace() {
      normalized_passport_info.push(string.to_string());
    }
  }
  if validate_passport(&normalized_passport_info) {
    valid_count += 1;
  }

  println!("{} valid passports", valid_count);
  Ok(())
}

fn validate_passport(passport_info: &Vec<String>) -> bool {
  let mut fields = HashMap::with_capacity(8);
  fields.insert("byr", Field::new(validate_byr));
  fields.insert("iyr", Field::new(validate_iyr));
  fields.insert("eyr", Field::new(validate_eyr));
  fields.insert("hgt", Field::new(validate_hgt));
  fields.insert("hcl", Field::new(validate_hcl));
  fields.insert("ecl", Field::new(validate_ecl));
  fields.insert("pid", Field::new(validate_pid));

  for string in passport_info {
    if let Some(field) = fields.get_mut(&string[0..3]) {
      field.validate(&string[4..]);
    }
  }

  fields.values().fold(true, |result, field| result && field.valid)
}

fn validate_byr(field: &str) -> bool {
  if let Ok(year) = field.parse::<i32>() {
    year >= 1920 && year <= 2002
  } else {
    false
  }
}

fn validate_iyr(field: &str) -> bool {
  if let Ok(year) = field.parse::<i32>() {
    year >= 2010 && year <= 2020
  } else {
    false
  }
}

fn validate_eyr(field: &str) -> bool {
  if let Ok(year) = field.parse::<i32>() {
    year >= 2020 && year <= 2030
  } else {
    false
  }
}

fn validate_hgt(field: &str) -> bool {
  let height_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
  if let Some(captures) = height_regex.captures(field) {
    let value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    if captures.get(2).unwrap().as_str() == "cm" {
      value >= 150 && value <= 193
    } else {
      value >= 59 && value <= 76
    }
  } else {
    false
  }
}

fn validate_hcl(field: &str) -> bool {
  let color_regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
  color_regex.is_match(field)
}

fn validate_ecl(field: &str) -> bool {
  match field {
    "amb" => true,
    "blu" => true,
    "brn" => true,
    "gry" => true,
    "grn" => true,
    "hzl" => true,
    "oth" => true,
    _ => false,
  }
}

fn validate_pid(field: &str) -> bool {
  let pid_regex = Regex::new(r"^\d{9}$").unwrap();
  pid_regex.is_match(field)
}

impl Field
{
  fn new(validation: fn(&str) -> bool) -> Field {
    Field { valid: false, validation: validation }
  }

  fn validate(&mut self, value: &str) {
    self.valid = (self.validation)(value);
  }
}
