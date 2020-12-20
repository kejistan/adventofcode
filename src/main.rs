use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufReader;
use regex::Regex;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

struct Image {
  data: Vec<Vec<bool>>,
  left_data: u32,
  right_data: u32,
  top_data: u32,
  bottom_data: u32,
}

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let name_regex = Regex::new(r"Tile (\d+):").unwrap();

  let mut images = HashMap::new();
  for g in reader.groups() {
    let group = g.unwrap();
    let mut lines = group.lines();
    let label = name_regex.captures(lines.next().unwrap()).unwrap()[1].parse::<u32>().unwrap();
    let data = lines.map(|l| l.chars().map(|c| {
      match c {
        '.' => false,
        '#' => true,
        _ => panic!(),
      }
    }).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    let mut left_data = 0;
    let mut right_data = 0;
    for row in data.iter() {
      left_data = left_data << 1;
      if row[0] {
        left_data += 1;
      }
      right_data = right_data << 1;
      if row[row.len() - 1] {
        right_data += 1;
      }
    }
    let mut top_data = 0;
    for b in data[0].iter() {
      top_data = top_data << 1;
      if *b {
        top_data += 1;
      }
    }
    let mut bottom_data = 0;
    for b in data[data.len() - 1].iter() {
      bottom_data = bottom_data << 1;
      if *b {
        bottom_data += 1;
      }
    }
    let image = Image {
      data: data,
      left_data: left_data,
      right_data: right_data,
      top_data: top_data,
      bottom_data: bottom_data,
    };

    images.insert(label, image);
  }

  let mut images_by_signatures: HashMap<u32, HashSet<u32>> = HashMap::new();

  for (label, image) in images.iter() {
    insert_image_by_signatures(&mut images_by_signatures, *label, image);
  }

  let mut corners = Vec::new();
  for (label, image) in images {
    let count = count_unmatched_sides(&images_by_signatures, &image);
    if count == 2 {
      corners.push(label);
    }
    if count > 2 {
      panic!();
    }
  }

  println!("corners: {:?}", corners);
  println!("{}", corners.into_iter().fold(1, |result, label| result * label as u64));

  Ok(())
}

fn count_unmatched_sides(images_by_signatures: &HashMap<u32, HashSet<u32>>, image: &Image) -> usize {
  let mut count = 0;
  if side_is_unmatched(images_by_signatures, image.left_data) {
    count += 1;
  }
  if side_is_unmatched(images_by_signatures, image.right_data) {
    count += 1;
  }
  if side_is_unmatched(images_by_signatures, image.top_data) {
    count += 1;
  }
  if side_is_unmatched(images_by_signatures, image.bottom_data) {
    count += 1;
  }

  count
}

fn side_is_unmatched(images_by_signatures: &HashMap<u32, HashSet<u32>>, signature: u32) -> bool {
  images_by_signatures.get(&signature).unwrap().len() == 1 && images_by_signatures.get(&flip_signature(signature)).unwrap().len() == 1
}

fn insert_image_by_signatures(images_by_signatures: &mut HashMap<u32, HashSet<u32>>, label: u32, image: &Image) {
  add_image_signature(images_by_signatures, label, image.left_data);
  add_image_signature(images_by_signatures, label, image.right_data);
  add_image_signature(images_by_signatures, label, image.top_data);
  add_image_signature(images_by_signatures, label, image.bottom_data);
  add_image_signature(images_by_signatures, label, flip_signature(image.left_data));
  add_image_signature(images_by_signatures, label, flip_signature(image.right_data));
  add_image_signature(images_by_signatures, label, flip_signature(image.top_data));
  add_image_signature(images_by_signatures, label, flip_signature(image.bottom_data));
}

fn add_image_signature(images_by_signatures: &mut HashMap<u32, HashSet<u32>>, label: u32, signature: u32) {
  if let Some(set) = images_by_signatures.get_mut(&signature) {
    set.insert(label);
  } else {
    let mut set = HashSet::new();
    set.insert(label);
    images_by_signatures.insert(signature, set);
  }
}

fn flip_signature(mut signature: u32) -> u32 {
  let mut flipped = 0;
  for _ in 0..10 {
    flipped = flipped << 1;
    flipped += signature & 1;
    signature = signature >> 1;
  }

  flipped
}
