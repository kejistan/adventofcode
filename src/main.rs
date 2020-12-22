use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum Player {
  One,
  Two,
}

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
    player1.push_back(line.parse::<u32>().unwrap());
  }

  lines.next(); // Player 2:
  while let Some(Ok(line)) = lines.next() {
    player2.push_back(line.parse::<u32>().unwrap());
  }

  let (_winner, score) = game(player1, player2);

  println!("{}", score);

  Ok(())
}

fn game(mut player1: VecDeque<u32>, mut player2: VecDeque<u32>) -> (Player, u64) {
  let mut history = HashSet::new();
  while !player1.is_empty() && !player2.is_empty() {
    let entry = (player1.clone(), player2.clone());
    if history.contains(&entry) {
      return (Player::One, 0)
    } else {
      history.insert(entry);
    }

    let card1 = player1.pop_front().unwrap();
    let card2 = player2.pop_front().unwrap();

    let winner;
    if player1.len() < card1 as usize || player2.len() < card2 as usize {
      if card1 > card2 {
        winner = Player::One;
      } else {
        winner = Player::Two;
      }
    } else {
      let mut sub1 = player1.iter().take(card1 as usize).map(|c| *c).collect::<VecDeque<u32>>();
      let mut sub2 = player2.iter().take(card2 as usize).map(|c| *c).collect::<VecDeque<u32>>();
      sub1.reserve(sub2.len());
      sub2.reserve(sub1.len());
      let (player, _) = game(sub1, sub2);
      winner = player;
    }

    match winner {
      Player::One => {
        player1.push_back(card1);
        player1.push_back(card2);
      },
      Player::Two => {
        player2.push_back(card2);
        player2.push_back(card1);
      }
    }
  }

  if !player1.is_empty() {
    (Player::One, score(player1))
  } else {
    (Player::Two, score(player2))
  }
}

fn score(mut deck: VecDeque<u32>) -> u64 {
  let slice = deck.make_contiguous();
  slice.reverse();
  slice.iter().enumerate().fold(0, |score, (i, card)| score + (i + 1) as u64 * *card as u64)
}
