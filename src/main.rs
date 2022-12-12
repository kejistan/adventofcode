use std::collections::VecDeque;
use std::{io};
use std::io::{BufReader, BufRead};

struct MapTile {
  elevation: u8,
  distance: Option<u32>,
}

type Coord = (usize, usize);

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let mut source = (0, 0);
  let mut destination = (0, 0);
  let mut x: usize = 0;
  let mut y: usize = 0;
  let mut map = input.lines().map(|result| {
    let line = result.unwrap();

    x = 0;
    let row = line.chars().into_iter().map(|char| {
      let tile = match char {
        'S' => {
          source = (x, y);
          MapTile::new(0)
        },
        'E' => {
          destination = (x, y);
          MapTile { elevation: 25, distance: Some(0) }
        }
        _ => {
          let elevation = char as u8 - 'a' as u8;
          MapTile::new(elevation)
        }
      };

      x += 1;
      tile
    }).collect::<Vec<MapTile>>();

    y += 1;
    row
  }).collect::<Vec<Vec<MapTile>>>();

  let mut queue = neighbors(&mut map, destination).into_iter().collect::<VecDeque<Coord>>();

  while queue.len() > 0 {
    let coordinate = queue.pop_front().unwrap();

    for coordinate in neighbors(&mut map, coordinate).into_iter() {
      queue.push_back(coordinate);
    }
  }

  let result = map.into_iter()
    .flat_map(|row| row.into_iter())
    .filter(|tile| tile.elevation == 0 && tile.distance.is_some())
    .min_by(|a, b| {
      a.distance.unwrap().cmp(&b.distance.unwrap())
    })
    .unwrap()
    .distance
    .unwrap();
  println!("{}", result);

  Ok(())
}

impl MapTile {
  fn new(elevation: u8) -> Self {
    MapTile { elevation, distance: None }
  }
}

fn neighbors(map: &mut Vec<Vec<MapTile>>, coordinate: Coord) -> Vec<Coord> {
  let mut coordinates = vec![];
  let mut offsets: Vec<(i8, i8)> = Vec::new();

  if coordinate.1 != 0 {
    offsets.push((0, -1));
  }
  if coordinate.1 != map.len() - 1 {
    offsets.push((0, 1));
  }
  if coordinate.0 != 0 {
    offsets.push((-1, 0));
  }
  if coordinate.0 != map[0].len() - 1 {
    offsets.push((1, 0));
  }

  let current_tile = &map[coordinate.1][coordinate.0];
  let elevation = current_tile.elevation;
  let distance = current_tile.distance.unwrap();
  
  for (x_offset, y_offset) in offsets.into_iter() {
    let x = (coordinate.0 as isize + x_offset as isize) as usize;
    let y = (coordinate.1 as isize + y_offset as isize) as usize;
    if map[y][x].elevation + 1 >= elevation && map[y][x].distance.is_none() {
      map[y][x].distance = Some(distance + 1);
      coordinates.push((x, y));
    }
  }

  coordinates
}
