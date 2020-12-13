use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());

  let buses = reader.lines().skip(1).next().unwrap().unwrap().split_terminator(',')
    .enumerate()
    .filter(|(_, bus)| *bus != "x")
    .map(|(i, bus)| (i, bus.parse::<i32>().unwrap()))
    .collect::<Vec<(usize, i32)>>();

    let mut stride = 1;
    let mut valid_timestamp: u64 = 0;

    for (offset, bus) in buses {
      let mut search_offset = 0;
      while (valid_timestamp + search_offset + offset as u64) % bus as u64 != 0 {
        search_offset += stride;
      }

      valid_timestamp += search_offset;
      stride *= bus as u64;
    }

    println!("{}", valid_timestamp);

    Ok(())
}
