use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
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

  let mut positions_to_check = Vec::with_capacity(rows.len());
  for row in rows.iter() {
    let mut pos_rows: Vec<Vec<(usize, usize)>> = Vec::with_capacity(row.len());
    for _ in 0..row.len() {
      pos_rows.push(vec![]);
    }
    positions_to_check.push(pos_rows);
  }

  let width = rows.first().unwrap().len();
  let mut prev_vertical: Vec<Option<usize>> = Vec::with_capacity(width);
  for _ in 0..width {
    prev_vertical.push(None);
  }

  for (y, row) in rows.iter().enumerate() {
    let seats_iterator = row.iter().enumerate().filter(|(_, pos)| **pos != Position::Floor);

    // horizontal scan
    let mut prev_horizontal: Option<usize> = None;
    for (x, _) in seats_iterator {
      match prev_horizontal {
        None => (),
        Some(prev_x) => {
          positions_to_check[y][prev_x].push((x, y));
          positions_to_check[y][x].push((prev_x, y));
        }
      }
      prev_horizontal = Some(x);

      // vertical scan
      match prev_vertical[x] {
        None => (),
        Some(prev_y) => {
          positions_to_check[prev_y][x].push((x, y));
          positions_to_check[y][x].push((x, prev_y));
        }
      }
      prev_vertical[x] = Some(y);

      // diagonal scans are dumb
      let mut next_x = x + 1;
      let mut next_y = y + 1;
      while next_x < row.len() && next_y < rows.len() {
        if rows[next_y][next_x] != Position::Floor {
          positions_to_check[y][x].push((next_x, next_y));
          positions_to_check[next_y][next_x].push((x, y));
          break;
        }

        next_x += 1;
        next_y += 1;
      }

      // bit of a hack, pre-decrement x for easier unsigned math
      next_x = x;
      next_y = y + 1;
      while next_x != 0 && next_y < rows.len() {
        next_x -= 1;

        if rows[next_y][next_x] != Position::Floor {
          positions_to_check[y][x].push((next_x, next_y));
          positions_to_check[next_y][next_x].push((x, y));
          break;
        }

        next_y += 1;
      }
    }
  }

  let mut stabilized = false;
  while !stabilized {
    let (changed, new_rows) = step(rows, &positions_to_check);
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

fn step(rows: Vec<Vec<Position>>, positions_to_check: &Vec<Vec<Vec<(usize, usize)>>>) -> (bool, Vec<Vec<Position>>) {
  let mut changed = false;
  let mut new_rows: Vec<Vec<Position>> = Vec::with_capacity(rows.len());
  for (y, row) in rows.iter().enumerate() {
    let mut new_row = Vec::with_capacity(row.len());
    for (x, pos) in row.iter().enumerate() {
      match pos {
        Position::Floor => new_row.push(Position::Floor),
        Position::Empty => {
          if count_occupied(&rows, &positions_to_check[y][x]) == 0 {
            changed = true;
            new_row.push(Position::Occupied)
          } else {
            new_row.push(Position::Empty)
          }
        },
        Position::Occupied => {
          if count_occupied(&rows, &positions_to_check[y][x]) >= 5 {
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

fn count_occupied(rows: &Vec<Vec<Position>>, positions_to_check: &Vec<(usize, usize)>) -> usize {
  let mut count = 0;

  for (x, y) in positions_to_check {
    if rows[*y][*x] == Position::Occupied {
      count += 1;
    }
  }

  count
}
