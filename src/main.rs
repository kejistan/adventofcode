use std::io;
use std::io::prelude::*;
use std::io::BufReader;

enum Position {
  Occupied,
  Empty,
  Floor,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let mut rows = reader.lines().map(|l|
    l.unwrap().chars().map(|c| {
      match c {
        'L' => Position::Empty,
        '#' => Position::Occupied,
        '.' => Position::Floor,
        _ => panic!(),
      }
    }).collect::<Vec<Position>>()
  ).collect::<Vec<Vec<Position>>>();

  let mut stabilized = false;
  while !stabilized {
    let (changed, new_rows) = step(rows);
    stabilized = !changed;
    rows = new_rows;
  }

  let count = rows.into_iter().fold(0, |count, row| {
    row.into_iter().fold(count, |count, pos| {
      match pos {
        Position::Occupied => count + 1,
        _ => count,
      }
    })
  });

  println!("{}", count);

  Ok(())
}

fn step(rows: Vec<Vec<Position>>) -> (bool, Vec<Vec<Position>>) {
  let mut changed = false;
  let mut new_rows: Vec<Vec<Position>> = Vec::with_capacity(rows.len());
  for (x, row) in rows.iter().enumerate() {
    let mut new_row = Vec::with_capacity(row.len());
    for (y, pos) in row.iter().enumerate() {
      match pos {
        Position::Floor => new_row.push(Position::Floor),
        Position::Empty => {
          if count_adjacent(x, y, &rows) == 0 {
            changed = true;
            new_row.push(Position::Occupied)
          } else {
            new_row.push(Position::Empty)
          }
        },
        Position::Occupied => {
          if count_adjacent(x, y, &rows) >= 4 {
            changed = true;
            new_row.push(Position::Empty)
          } else {
            new_row.push(Position::Occupied)
          }
        }
      }
    }
    new_rows.push(new_row);
  }

  (changed, new_rows)
}

fn count_adjacent(x: usize, y: usize, rows: &Vec<Vec<Position>>) -> usize {
  let mut count = 0;
  let min_x = if x == 0 { x } else { x - 1 };
  let max_x = if x + 1 == rows.len() { x } else { x + 1 };
  let min_y = if y == 0 { y } else { y - 1 };
  let max_y = if y + 1 == rows[x].len() { y } else { y + 1 };

  for (offset_x, row) in rows[min_x..=max_x].iter().enumerate() {
    for (offset_y, pos) in row[min_y..=max_y].iter().enumerate() {
      if min_x + offset_x == x && min_y + offset_y == y {
        continue;
      }

      match pos {
        Position::Occupied => count += 1,
        _ => (),
      }
    }
  }
  
  count
}

fn print_rows(rows: &[Vec<Position>]) {
  for row in rows {
    for pos in row {
      print!("{}", pos);
    }
    println!("");
  }
}

impl std::fmt::Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let character = match self {
      Position::Empty => 'L',
      Position::Floor => '.',
      Position::Occupied => '#',
    };
    write!(f, "{}", character)
  }
}