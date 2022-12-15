use std::ops::{AddAssign, Sub, Add};
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Coordinate {
  pub x: i32,
  pub y: i32,
}

impl Coordinate {
  pub fn new(x: i32, y: i32) -> Coordinate {
    Coordinate { x, y }
  }
}

impl Debug for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("({},{})", self.x, self.y))
  }
}

impl Add for Coordinate {
  type Output = Self;

  fn add(self, other: Self) -> Coordinate {
    Coordinate::new(self.x + other.x, self.y + other.y)
  }
}

impl AddAssign for Coordinate {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl Sub for &Coordinate {
  type Output = Coordinate;

  fn sub(self, other: Self) -> Coordinate {
    Coordinate::new(self.x - other.x, self.y - other.y)
  }
}

impl Sub for Coordinate {
  type Output = Self;

  fn sub(self, other: Self) -> Coordinate {
    Coordinate::new(self.x - other.x, self.y - other.y)
  }
}