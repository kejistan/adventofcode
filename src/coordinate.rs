use std::ops::{AddAssign, Sub};

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