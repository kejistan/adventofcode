use std::collections::HashMap;
use std::io;
use std::io::BufReader;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let total = reader.groups().map(|g| {
    let group = g.unwrap();
    let mut group_size = 0;
    let mut answers: HashMap<char, i32> = HashMap::new();
    for line in group.lines() {
      group_size += 1;
      for c in line.chars() {
        if let Some(count) = answers.get_mut(&c) {
          *count += 1;
        } else {
          answers.insert(c, 1);
        }
      }
    }

    answers.iter().filter(|(_, &count)| count == group_size).count()
  }).fold(0, |total, count| count + total);

  println!("Total answers: {}", total);
  Ok(())
}
