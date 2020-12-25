use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let public_keys = input.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect::<Vec<u32>>();
  let loop_sizes = public_keys.iter().map(brute_force_key).collect::<Vec<u32>>();

  let encryption_key = derive_key(public_keys[0], loop_sizes[1]);
  println!("{}", encryption_key);

  Ok(())
}

fn brute_force_key(key: &u32) -> u32 {
  let mut value = 7;
  let mut loop_size = 0;
  while value != *key {
    value = ((value as u64 * 7) % 20201227) as u32;
    loop_size += 1;
  }

  loop_size
}

fn derive_key(pubkey: u32, loop_size: u32) -> u32 {
  let mut value = pubkey;
  for _ in 0..loop_size {
    value = ((value as u64 * pubkey as u64) % 20201227) as u32;
  }

  value
}
