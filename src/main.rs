use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type Coordinate = (i32, i32, i32);

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let mut universe = HashSet::new();
  for (y, l) in reader.lines().enumerate() {
    let line = l.unwrap();
    for (x, c) in line.chars().enumerate() {
      match c {
        '#' => {
          universe.insert((x as i32, y as i32, 0));
        }
        _ => (),
      }
    }
  }

  for _ in 0..6 {
    let mut active_counts = HashMap::new();
    for coordinate in universe.iter() {
      increment_neighbors(&mut active_counts, *coordinate);
    }

    universe = active_counts.into_iter().filter(|(coordinate, num)| {
      if universe.contains(coordinate) {
        *num == 2 || *num == 3
      } else {
        *num == 3
      }
    }).map(|(coord, _)| coord)
    .collect::<HashSet<Coordinate>>();
  }

  println!("{}", universe.len());

  Ok(())
}

fn increment_neighbors(counts: &mut HashMap<Coordinate, u8>, coordinate: Coordinate) {
  for coord in neighbors(coordinate) {
    if let Some(num) = counts.get_mut(&coord) {
      *num += 1;
    } else {
      counts.insert(coord, 1);
    }
  }
}

fn neighbors((x, y, z): Coordinate) -> impl std::iter::Iterator<Item=Coordinate> {
  (z-1..=z+1).flat_map(move |nz| {
    (y-1..=y+1).flat_map(move |ny| {
      (x-1..=x+1).map(move |nx| (nx, ny, nz))
    })
  }).filter(move |(nx, ny, nz)| x != *nx || y != *ny || z != *nz)
}
