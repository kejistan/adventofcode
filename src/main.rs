use std::io;
use std::io::prelude::*;
use std::io::BufReader;

struct Slope {
  x: usize,
  y: usize,
}

struct TrajectoryState {
  slope: Slope,
  x: usize,
  y: usize,
  trees: i32,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let slopes = vec![
    Slope {
      x: 1,
      y: 1,
    },
    Slope {
      x: 3,
      y: 1,
    },
    Slope {
      x: 5,
      y: 1,
    },
    Slope {
      x: 7,
      y: 1,
    },
    Slope {
      x: 1,
      y: 2,
    },
  ];

  let trees = reader.lines().map(Result::unwrap).map(|line| {
    line.chars().map(|c| c != '.').collect::<Vec<bool>>()
  }).collect::<Vec<Vec<bool>>>();

  let result = slopes.into_iter().map(|slope| TrajectoryState {
    slope: slope,
    x: 0,
    y: 0,
    trees: 0,
  }).map(|mut trajectory| {
    while trajectory.y < trees.len() {
      let row = &trees[trajectory.y];
      trajectory.advance(row[trajectory.x % row.len()])
    }

    println!("{} trees on slope ({}, {})", trajectory.trees, trajectory.slope.x, trajectory.slope.y);
    trajectory.trees
  }).fold(1, |product, count| product * count);

  println!("result {}", result);
  Ok(())
}

impl TrajectoryState {
  fn advance(&mut self, hit_tree: bool) {
    if hit_tree {
      self.trees += 1;
    }

    self.x += self.slope.x;
    self.y += self.slope.y;
  }
}
