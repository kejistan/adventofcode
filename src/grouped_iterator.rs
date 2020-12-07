use std::io::Lines;
use std::io::Result;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

pub struct Groups<B> {
  lines: Lines<B>,
}

impl<B: BufRead> Iterator for Groups<B> {
  type Item = Result<String>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut group: Vec<String> = vec![];

    while let Some(l) = self.lines.next() {
      if let Ok(line) = l {
        if line.is_empty() {
          return Some(Ok(group.join("\n")));
        }
        group.push(line);
      } else {
        return Some(l);
      }
    }

    if group.is_empty() {
      None
    } else {
      Some(Ok(group.join("\n")))
    }
  }
}

pub trait GroupedIterator<B> {
  fn groups(self) -> Groups<B>;
}

impl<R: Read> GroupedIterator<BufReader<R>> for BufReader<R> {
  fn groups(self) -> Groups<BufReader<R>> {
    Groups { lines: self.lines() }
  }
}