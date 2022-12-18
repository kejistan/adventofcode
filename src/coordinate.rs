use std::ops::{AddAssign, Sub, Add};
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Coordinate<T = i32> {
  pub x: T,
  pub y: T,
}

impl<T> Coordinate<T> {
  pub fn new(x: T, y: T) -> Coordinate<T> {
    Coordinate { x, y }
  }
}

impl<T: std::fmt::Display> Debug for Coordinate<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("({},{})", self.x, self.y))
  }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
  type Output = Self;

  fn add(self, other: Self) -> Coordinate<T> {
    Coordinate::new(self.x + other.x, self.y + other.y)
  }
}

impl<T: Add<Output = T> + Copy> Add for &Coordinate<T> {
  type Output = Coordinate<T>;

  fn add(self, other: Self) -> Coordinate<T> {
    Coordinate::new(self.x + other.x, self.y + other.y)
  }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl<T: Sub<Output = T> + Copy> Sub for &Coordinate<T> {
  type Output = Coordinate<T>;

  fn sub(self, other: Self) -> Coordinate<T> {
    Coordinate::new(self.x - other.x, self.y - other.y)
  }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
  type Output = Self;

  fn sub(self, other: Self) -> Coordinate<T> {
    Coordinate::new(self.x - other.x, self.y - other.y)
  }
}