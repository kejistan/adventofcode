use std::collections::{VecDeque, HashMap};
use std::{io};
use std::io::{BufReader};

mod command_iterator;
use command_iterator::CommandsIterator;

use crate::command_iterator::{Command, DirEnt};

struct DirectoryInfo {
  has_listed: bool,
  size: usize,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());

  let mut directories: HashMap<String, DirectoryInfo> = HashMap::new();
  let mut current_dir_stack: VecDeque<String> = VecDeque::new();
  for command in input.commands() {
    match command {
      Command::Cd { to } if to == "/" => {
        current_dir_stack.clear();
        current_dir_stack.push_back("".to_string());
      },
      Command::Cd { to } if to == ".." => {
        current_dir_stack.pop_back();
      },
      Command::Cd { to } => {
        let mut full_path = current_dir_stack.back().unwrap().clone();
        full_path.push_str("/");
        full_path.push_str(&to);
        current_dir_stack.push_back(full_path);
      },
      Command::Ls { contents } => {
        let cwd = current_dir_stack.back().unwrap();
        if let Some(DirectoryInfo { has_listed, size: _ }) = directories.get(cwd) {
          if *has_listed {
            break;
          }
        }

        let mut dir_size = 0;
        for entry in contents {
          match entry {
            DirEnt::File { name: _, size } => dir_size += size,
            _ => {},
          }
        }

        for path in &current_dir_stack {
          let is_current_dir = *path == *cwd;
          directories.entry(path.clone()).and_modify(|info| {
            info.size += dir_size;
          }).or_insert(DirectoryInfo { has_listed: is_current_dir, size: dir_size });
        }
      },
    }
  }

  let mut result = 0;
  for DirectoryInfo {size, has_listed: _ } in directories.values() {
    if *size <= 100_000 {
      result += size;
    }
  }

  println!("{}", result);

  Ok(())
}
