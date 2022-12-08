use std::cmp::max;
use std::{io};
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let trees = input.lines().map(|result| {
    let line = result.unwrap();
    line.bytes().map(|byte| {
      byte - '0' as u8
    }).collect::<Vec<u8>>()
  }).collect::<Vec<Vec<u8>>>();

  let mut max_score: u32 = 0;

  for y in 0..trees.len() {
    let row = &trees[y];
    for x in 0..row.len() {
      let height = row[x];

      let mut w_distance = 0;
      let mut sight_x = x;
      while sight_x > 0 {
        sight_x -= 1;
        w_distance += 1;
        if row[sight_x] >= height {
          break;
        }
      }

      let mut e_distance = 0;
      sight_x = x;
      while sight_x < row.len() - 1 {
        sight_x += 1;
        e_distance += 1;
        if row[sight_x] >= height {
          break;
        }
      }

      let mut n_distance = 0;
      let mut sight_y = y;
      while sight_y > 0 {
        sight_y -= 1;
        n_distance += 1;
        if trees[sight_y][x] >= height {
          break;
        }
      }

      let mut s_distance = 0;
      sight_y = y;
      while sight_y < trees.len() - 1 {
        sight_y += 1;
        s_distance += 1;
        if trees[sight_y][x] >= height {
          break;
        }
      }

      let score = w_distance * e_distance * n_distance * s_distance;
      max_score = max(score, max_score);
    }
  }

  println!("{}", max_score);

  Ok(())
}
