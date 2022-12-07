use std::io::Lines;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::iter::Peekable;

pub struct Commands<B: BufRead> {
  lines: Peekable<Lines<B>>,
}

pub enum DirEnt {
  Dir { name: String },
  File { name: String, size: usize },
}

pub enum Command {
  Cd {
    to: String,
  },
  Ls {
    contents: Vec<DirEnt>,
  }
}

impl<B: BufRead> Iterator for Commands<B> {
  type Item = Command;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(result) = self.lines.next() {
      let line = result.unwrap();
      let (_, command) = line.split_at(2);
      if command.starts_with("cd") {
        let (_, to) = command.split_at("cd".len() + 1);
        return Some(Command::Cd { to: to.to_string() });
      }

      let mut contents: Vec<DirEnt> = Vec::new();
      while self.lines.peek().is_some() && !self.lines.peek().unwrap().as_ref().unwrap().starts_with('$') {
        let line = self.lines.next().unwrap().unwrap();
        let (size_or_dir, name) = line.split_once(' ').unwrap();
        if size_or_dir == "dir" {
          contents.push(DirEnt::Dir { name: name.to_string() })
        } else {
          let size = size_or_dir.parse::<usize>().unwrap();
          let name = name.to_string();

          contents.push(DirEnt::File { name, size });
        }
      }

      return Some(Command::Ls { contents })
    }

    None
  }
}

pub trait CommandsIterator<B: BufRead> {
  fn commands(self) -> Commands<B>;
}

impl<R: Read> CommandsIterator<BufReader<R>> for BufReader<R> {
  fn commands(self) -> Commands<BufReader<R>> {
    Commands { lines: self.lines().peekable() }
  }
}