use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufReader;
use regex::Regex;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

#[derive(PartialEq, Copy, Clone)]
enum Pixel {
  Empty,
  Wave,
  Monster,
}

struct Image {
  label: u32,
  data: Vec<Vec<Pixel>>,
}

struct ImageTile<'a> {
  image: ImageRef<'a>,
  signatures: Vec<u16>,
}

struct ImageRef<'a> {
  image: &'a Image,
  orientation: Orientation,
}

#[derive(Debug)]
struct Orientation {
  flip_vertical: bool,
  flip_horizontal: bool,
  rotate: u16,
}

#[derive(Debug)]
struct ImageRow<'a>(Vec<ImageTile<'a>>);

type SignatureMap = HashMap<u16, HashSet<u32>>;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
 
  let composite = build_composite(reader);
  let mut orientable_composite = ImageRef::with_default_orientation(&composite);
  // println!("{}", orientable_composite);

  let result = count_non_monsters(&mut orientable_composite);
  println!("{}", result);

  Ok(())
}

fn count_non_monsters<'a>(image: &mut ImageRef<'a>) -> u64 {
  let mut total_count = 0;
  for y in 0..image.dimension() {
    for x in 0..image.dimension() {
      if image.get(y, x) == Pixel::Wave {
        total_count += 1;
      }
    }
  }

  let mut monster_count = 0;
  while monster_count == 0 {
    monster_count = count_monster_parts(image);

    if monster_count == 0 {
      image.flip_vertical();
      monster_count = count_monster_parts(image);
    }

    if monster_count == 0 {
      image.flip_vertical();
      image.flip_horizontal();
      monster_count = count_monster_parts(image);
    }

    if monster_count == 0 {
      image.flip_horizontal();
      image.rotate(1);
    }
  }

  println!("total: {}", total_count);
  println!("monster: {}", monster_count);

  total_count - monster_count
}

fn count_monster_parts<'a>(image: &mut ImageRef<'a>) -> u64 {
  let mut monster_parts = HashSet::new();
  for y in 0..image.dimension() {
    for x in 0..image.dimension() {
      add_monster_parts(image, x, y, &mut monster_parts);
    }
  }

  let count = monster_parts.len() as u64;

  if count > 0 {
    let image = render_monster_parts(image, &monster_parts);
    println!("{}", ImageRef::with_default_orientation(&image));
    println!("");
  }

  count
}

fn render_monster_parts<'a>(image: &ImageRef<'a>, parts: &HashSet<(usize, usize)>) -> Image {
  let mut data = Vec::with_capacity(image.dimension());
  for y in 0..image.dimension() {
    let mut row = Vec::with_capacity(image.dimension());
    for x in 0..image.dimension() {
      if parts.contains(&(x, y)) {
        row.push(Pixel::Monster);
      } else {
        row.push(image.get(y, x));
      }
    }
    data.push(row);
  }
  
  Image {
    data,
    label: 0,
  }
}

fn add_monster_parts<'a>(image: &ImageRef<'a>, x: usize, y: usize, parts: &mut HashSet<(usize, usize)>) {
  let dimension = image.dimension();
  let monster = vec![
    "                  # ".chars().map(|c| c == '#').collect::<Vec<bool>>(),
    "#    ##    ##    ###".chars().map(|c| c == '#').collect::<Vec<bool>>(),
    " #  #  #  #  #  #   ".chars().map(|c| c == '#').collect::<Vec<bool>>(),
  ];
  if x + monster[0].len() > dimension {
    return;
  }
  if y + monster.len() > dimension {
    return;
  }

  let mut coordinates = vec![];
  for (row_i, row) in monster.iter().enumerate() {
    for (col_i, &col) in row.iter().enumerate() {
      if col {
        coordinates.push((x + col_i, y + row_i))
      }
    }
  }

  if coordinates.iter().all(|(x, y)| image.get(*y, *x) == Pixel::Wave) {
    for coord in coordinates.into_iter() {
      parts.insert(coord);
    }
  }
}

fn build_composite<R: std::io::Read>(reader: BufReader<R>) -> Image {
  let images = collect_images(reader);

  let mut images_by_signatures: SignatureMap = HashMap::new();

  for (label, image) in images.iter() {
    insert_image_by_signatures(&mut images_by_signatures, *label, &ImageTile::with_default_orientation(image));
  }

  let mut composite: Vec<ImageRow> = Vec::new();

  // find and orient a "top left" corner
  let (_, image) = images.iter().find(|(_, image)| {
    count_unmatched_sides(&images_by_signatures, &ImageTile::with_default_orientation(image)) == 2
  }).unwrap();
  let image = ImageTile::with_default_orientation(image);
  let mut orientation = Orientation {
    flip_vertical: false,
    flip_horizontal: false,
    rotate: 0,
  };
  let mut interior_sides = image.signatures.iter().enumerate().filter(|(_, &sig)| {
    !side_is_unmatched(&images_by_signatures, sig)
  }).map(|(i, _)| i).collect::<Vec<usize>>();
  interior_sides.sort();
  match (interior_sides[0], interior_sides[1]) {
    (0, 3) => (), // already oriented
    (x, y) if x + 1 == y => {
      orientation.rotate = (3 - x) as u16;
    },
    _ => {
      panic!();
    }
  }
  for side in interior_sides {
    let signature = image.signatures[side];
    if images_by_signatures.get(&signature).unwrap().len() == 1 {
      // there must be a flip involved
      if images_by_signatures.get(&flip_signature(signature)).unwrap().len() != 2 {
        panic!();
      }

      match (side + orientation.rotate as usize) % 4 {
        0 => {
          orientation.flip_vertical = true;
          orientation.rotate = (orientation.rotate + 3) % 4;
        },
        3 => {
          orientation.flip_horizontal = true;
          orientation.rotate = (orientation.rotate + 1) % 4;
        },
        _ => panic!(),
      }
    }
  }

  let corner = ImageTile::new(image.image.image, orientation);

  composite.push(ImageRow(vec![corner]));
  loop {
    loop {
      let (row, prev_rows) = composite.split_last_mut().unwrap();
      let mut left_signature = None;
      let mut top_signature = None;
      let mut image = None;
      if let Some(left) = row.last() {
        left_signature = Some(left.right());
        if let Some(label) = find_match(&images_by_signatures, left.label(), left_signature.unwrap()) {
          image = images.get(&label);
        } else {
          // This should happen when we reach the end of a row
          break;
        }
      }
      if !prev_rows.is_empty() {
        let top = &prev_rows.last().unwrap()[row.len()];
        top_signature = Some(top.bottom());
        if let Some(label) = find_match(&images_by_signatures, top.label(), top_signature.unwrap()) {
          if image.is_none() {
            image = images.get(&label);
          }
        } else {
          // This should happen when we reach the end of the composite
          break;
        }
      }

      row.push(orient(left_signature, top_signature, image.unwrap()));
    }

    if composite.last().unwrap().is_empty() {
      composite.pop();
      break;
    } else {
      composite.push(ImageRow(vec![]));
    }
  }

  let composite_dim = composite.len() * 8;
  let mut data = Vec::with_capacity(composite_dim);
  for row in composite {
    for y in 1..9 {
      let mut data_row = Vec::with_capacity(composite_dim);
      for image in row.iter() {
        for x in 1..(image.dimension() - 1) {
          data_row.push(image.get(y, x));
        }
      }
      data.push(data_row);
    }
  }

  Image {
    data,
    label: 0,
  }
}

fn orient<'a>(mut left: Option<u16>, mut top: Option<u16>, image: &'a Image) -> ImageTile<'a> {
  // comparing opposite sides requires flipping the signature, do the flip here for simplicity
  left = left.map(flip_signature);
  top = top.map(flip_signature);

  let mut image = ImageTile::with_default_orientation(image);
  if let Some(left) = left {
    let (i, _) = image.signatures.iter().enumerate().find(|(_, &sig)| {
      sig == left || flip_signature(sig) == left
    }).unwrap();
    match i {
      0 => image.rotate(2),
      1 => image.rotate(1),
      2 => (),
      3 => image.rotate(3),
      _ => panic!(),
    };

    if left != image.left() {
      image.flip_vertical();
      if left != image.left() {
        panic!();
      }
    }
  }
  
  if let Some(top) = top {
    let (i, _) = image.signatures.iter().enumerate().find(|(_, &sig)| {
      sig == top || flip_signature(sig) == top
    }).unwrap();
    match i {
      0 => image.rotate(1),
      1 => (),
      2 => image.rotate(3),
      3 => image.rotate(2),
      _ => panic!(),
    };

    if top != image.top() {
      image.flip_horizontal();
      if top != image.top() {
        panic!();
      }
    }
  }

  if let Some(left) = left {
    if left != image.left() {
      panic!();
    }
  }

  image
}

fn find_match(images_by_signatures: &SignatureMap, label: u32, signature: u16) -> Option<u32> {
  let matches = images_by_signatures.get(&signature).unwrap().iter().filter(|&&l| l != label).collect::<Vec<&u32>>();
  if matches.len() > 1 {
    panic!();
  }

  if matches.is_empty() {
    None
  } else {
    Some(*matches[0])
  }
}

fn count_unmatched_sides(images_by_signatures: &SignatureMap, image: &ImageTile) -> usize {
  image.signatures.iter().filter(|&&signature| side_is_unmatched(images_by_signatures, signature)).count()
}

fn side_is_unmatched(images_by_signatures: &SignatureMap, signature: u16) -> bool {
  images_by_signatures.get(&signature).unwrap().len() == 1
}

fn insert_image_by_signatures(images_by_signatures: &mut SignatureMap, label: u32, image: &ImageTile) {
  for signature in image.signatures.iter() {
    add_image_signature(images_by_signatures, label, *signature);
    add_image_signature(images_by_signatures, label, flip_signature(*signature));
  }
}

fn add_image_signature(images_by_signatures: &mut SignatureMap, label: u32, signature: u16) {
  if let Some(set) = images_by_signatures.get_mut(&signature) {
    set.insert(label);
  } else {
    let mut set = HashSet::new();
    set.insert(label);
    images_by_signatures.insert(signature, set);
  }
}

fn flip_signature(mut signature: u16) -> u16 {
  let mut flipped = 0;
  for _ in 0..10 {
    flipped = flipped << 1;
    flipped += signature & 1;
    signature = signature >> 1;
  }

  flipped
}

fn collect_images<R: std::io::Read>(reader: BufReader<R>) -> HashMap<u32, Image> {
  let name_regex = Regex::new(r"Tile (\d+):").unwrap();
  let mut images = HashMap::new();
  for g in reader.groups() {
    let group = g.unwrap();
    let mut lines = group.lines();
    let label = name_regex.captures(lines.next().unwrap()).unwrap()[1].parse::<u32>().unwrap();
    let data = lines.map(|l| l.chars().map(|c| {
      match c {
        '.' => Pixel::Empty,
        '#' => Pixel::Wave,
        _ => panic!(),
      }
    }).collect::<Vec<Pixel>>()).collect::<Vec<Vec<Pixel>>>();

    let image = Image {
      label,
      data,
    };

    images.insert(label, image);
  }

  images
}

impl ImageTile<'_> {
  fn with_default_orientation<'a>(image: &'a Image) -> ImageTile<'a> {
    ImageTile::new(image, Orientation {
      flip_vertical: false,
      flip_horizontal: false,
      rotate: 0,
    })
  }

  fn new<'a>(image: &'a Image, orientation: Orientation) -> ImageTile<'a> {
    let image = ImageRef::new(image, orientation);

    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;
    let dimension = image.image.data.len();
    for i in 0..dimension {
      top = top << 1;
      right = right << 1;

      if image.get(0, i) == Pixel::Wave {
        top += 1;
      }
      // bottom is calculated as little endian, this simplifies rotations
      if image.get(dimension - 1, i) == Pixel::Wave {
        bottom |= 1 << i;
      }
      // left is calculated as little endian, this simplifies rotations
      if image.get(i, 0) == Pixel::Wave {
        left |= 1 << i;
      }
      if image.get(i, dimension - 1) == Pixel::Wave {
        right += 1;
      }
    }

    let signatures = vec![right, top, left, bottom];

    ImageTile {
      image,
      signatures,
    }
  }

  fn label(&self) -> u32 {
    self.image.image.label
  }

  fn get(&self, row: usize, col: usize) -> Pixel {
    self.image.get(row, col)
  }

  fn dimension(&self) -> usize {
    self.image.dimension()
  }

  fn left(&self) -> u16 {
    self.signatures[2]
  }

  fn right(&self) -> u16 {
    self.signatures[0]
  }

  fn top(&self) -> u16 {
    self.signatures[1]
  }

  fn bottom(&self) -> u16 {
    self.signatures[3]
  }

  fn rotate(&mut self, rotation: u16) {
    self.image.rotate(rotation);
    self.signatures.rotate_right(rotation as usize);
  }

  fn flip_horizontal(&mut self) {
    self.image.flip_horizontal();
    self.signatures.swap(0, 2);
    for signature in self.signatures.iter_mut() {
      *signature = flip_signature(*signature);
    }
  }

  fn flip_vertical(&mut self) {
    self.image.flip_vertical();
    self.signatures.swap(1, 3);
    for signature in self.signatures.iter_mut() {
      *signature = flip_signature(*signature);
    }
  }
}

impl ImageRef<'_> {
  fn with_default_orientation<'a>(image: &'a Image) -> ImageRef<'a> {
    ImageRef::new(image, Orientation {
      flip_vertical: false,
      flip_horizontal: false,
      rotate: 0,
    })
  }

  fn new<'a>(image: &'a Image, orientation: Orientation) -> ImageRef<'a> {
    ImageRef {
      image,
      orientation,
    }
  }

  fn dimension(&self) -> usize {
    self.image.data.len()
  }

  fn get(&self, row: usize, col: usize) -> Pixel {
    let (y, x) = self.transform(row, col);

    self.image.data[y][x]
  }

  fn transform(&self, mut row: usize, mut col: usize) -> (usize, usize) {
    let max_dimension = self.image.data.len() - 1;
    if self.orientation.flip_horizontal {
      col = max_dimension - col;
    }
    if self.orientation.flip_vertical {
      row = max_dimension - row;
    }

    let x;
    let y;
    match self.orientation.rotate {
      0 => {
        x = col;
        y = row;
      },
      1 => {
        x = max_dimension - row;
        y = col;
      },
      2 => {
        x = max_dimension - col;
        y = max_dimension - row;
      },
      3 => {
        x = row;
        y = max_dimension - col;
      },
      _ => panic!(),
    }

    (y, x)
  }

  fn rotate(&mut self, rotation: u16) {
    self.orientation.rotate = (self.orientation.rotate + rotation) % 4;
  }

  fn flip_horizontal(&mut self) {
    self.orientation.flip_horizontal = !self.orientation.flip_horizontal;
  }

  fn flip_vertical(&mut self) {
    self.orientation.flip_vertical = !self.orientation.flip_vertical;
  }
}

impl<'a> std::fmt::Display for ImageRef<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let dimension = self.image.data.len();
    for row in 0..dimension {
      for col in 0..dimension {
        write!(f, "{}", self.get(row, col))?;
      }
      if row != dimension - 1 {
        write!(f, "\n")?;
      }
    }

    Ok(())
  }
}

impl std::fmt::Display for Pixel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Pixel::Empty => '.',
      Pixel::Wave => '#',
      Pixel::Monster => 'O',
    })
  }
}

impl<'a> std::fmt::Display for ImageTile<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.image)
  }
}

impl<'a> std::fmt::Display for ImageRow<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let dimension = self.0[0].image.image.data.len();
    for row in 0..dimension {
      for image in self.0.iter() {
        for col in 0..dimension {
          write!(f, "{}", image.get(row, col))?;
        }
        write!(f, " ")?;
      }
      if row != dimension - 1 {
        write!(f, "\n")?;
      }
    }

    Ok(())
  }
}

impl<'a> std::ops::Deref for ImageRow<'a> {
  type Target = Vec<ImageTile<'a>>;

  fn deref(&self) -> &Vec<ImageTile<'a>> {
    &self.0
  }
}

impl<'a> std::ops::DerefMut for ImageRow<'a> {
  fn deref_mut(&mut self) -> &mut Vec<ImageTile<'a>> {
    &mut self.0
  }
}

impl<'a> std::fmt::Debug for ImageRef<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Ref {{ {:?}, {:?} }}", self.image.label, self.orientation)
  }
}

impl<'a> std::fmt::Debug for ImageTile<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Tile {{ {:?}, {:?} }}", self.image, self.signatures)
  }
}
