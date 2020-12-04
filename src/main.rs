use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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
  fields.insert("byr", false);
  fields.insert("iyr", false);
  fields.insert("eyr", false);
  fields.insert("hgt", false);
  fields.insert("hcl", false);
  fields.insert("ecl", false);
  fields.insert("pid", false);
  fields.insert("cid", true);

  for string in passport_info {
    if let Some(val) = fields.get_mut(&string[0..3]) {
      *val = true;
    }
  }

  fields.values().fold(true, |result, &field| result && field)
}
