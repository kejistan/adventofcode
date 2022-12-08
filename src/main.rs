use std::cmp::max;
use std::{io};
use std::io::{BufReader, BufRead};

struct TreeState {
  height: u8,
  max_n: Option<u8>,
  max_s: Option<u8>,
  max_e: Option<u8>,
  max_w: Option<u8>,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let trees = input.lines().map(|result| {
    let line = result.unwrap();
    line.bytes().map(|byte| {
      byte - '0' as u8
    }).collect::<Vec<u8>>()
  }).collect::<Vec<Vec<u8>>>();

  let mut trees_with_info: Vec<Vec<TreeState>> = Vec::new();

  for y in 0..trees.len() {
    let row = &trees[y];
    let mut row_with_info: Vec<TreeState> = Vec::with_capacity(row.len());

    for x in 0..row.len() {
      let height = row[x];

      let mut max_w = None;
      if x > 0 {
        let west_tree_info = &row_with_info[x - 1];
        if let Some(max_tree) = west_tree_info.max_w {
          max_w = Some(max(max_tree, west_tree_info.height));
        } else {
          max_w = Some(west_tree_info.height);
        }
      }

      let mut max_n = None;
      if y > 0 {
        let north_tree_info = &trees_with_info[y - 1][x];
        if let Some(max_tree) = north_tree_info.max_n {
          max_n = Some(max(max_tree, north_tree_info.height));
        } else {
          max_n = Some(north_tree_info.height);
        }
      }

      row_with_info.push(TreeState { height, max_n, max_s: None, max_e: None, max_w });
    }

    trees_with_info.push(row_with_info);
  }

  for y_offset in 1..=trees.len() {
    let y = trees.len() - y_offset;
    let row = &trees[y];

    for x_offset in 1..=row.len() {
      let x = row.len() - x_offset;

      if x + 1 < row.len() {
        let east_tree_info = &trees_with_info[y][x + 1];
        if let Some(max_tree) = east_tree_info.max_e {
          trees_with_info[y][x].max_e = Some(max(max_tree, east_tree_info.height));
        } else {
          trees_with_info[y][x].max_e = Some(east_tree_info.height);
        }
      }

      if y + 1 < trees.len() {
        let south_tree_info = &trees_with_info[y + 1][x];
        if let Some(max_tree) = south_tree_info.max_s {
          trees_with_info[y][x].max_s = Some(max(max_tree, south_tree_info.height));
        } else {
          trees_with_info[y][x].max_s = Some(south_tree_info.height);
        }
      }
    }
  }

  let result = trees_with_info.iter().flat_map(|trees| {
    trees.iter().map(|tree| !tree.is_hidden())
  }).filter(|boolean| *boolean).count();

  println!("{}", result);

  Ok(())
}

impl TreeState {
  fn is_hidden(&self) -> bool {
    if self.max_e.is_none() || self.max_n.is_none() || self.max_s.is_none() || self.max_w.is_none() {
      return false;
    }

    self.height <= [
      self.max_e.unwrap(),
      self.max_n.unwrap(),
      self.max_s.unwrap(),
      self.max_w.unwrap()
    ].into_iter().min().unwrap()
  }
}
