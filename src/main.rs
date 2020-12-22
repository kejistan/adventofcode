use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
  let reader = BufReader::new(io::stdin());
  let mut lines = reader.lines();
  lines.next(); // Player 1:

  let mut player1 = VecDeque::new();
  let mut player2 = VecDeque::new();
  while let Some(Ok(line)) = lines.next() {
    if line.is_empty() {
      break;
    }
    player1.push_front(line.parse::<u32>().unwrap());
  }

  lines.next(); // Player 2:
  while let Some(Ok(line)) = lines.next() {
    player2.push_front(line.parse::<u32>().unwrap());
  }

  while !player1.is_empty() && !player2.is_empty() {
    let card1 = player1.pop_back().unwrap();
    let card2 = player2.pop_back().unwrap();

    if card1 > card2 {
      player1.push_front(card1);
      player1.push_front(card2);
    } else {
      player2.push_front(card2);
      player2.push_front(card1);
    }
  }

  let winning_deck = if !player1.is_empty() {
    player1
  } else {
    player2
  };

  let score = winning_deck.into_iter().enumerate().fold(0, |score, (i, card)| score + (i + 1) as u64 * card as u64);
  println!("{}", score);

  Ok(())
}
