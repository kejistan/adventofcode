use std::io;
use std::io::BufReader;

mod grouped_iterator;
use grouped_iterator::GroupedIterator;

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let mut groups = input.groups();
  let numbers = groups.next().unwrap()?.split(',').map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>();

  let mut boards = Vec::with_capacity(5);

  for input in groups {
    let mut board = Vec::with_capacity(5);
    for line in input.unwrap().lines() {
      board.push(line.split_ascii_whitespace().map(|num| (num.parse::<u32>().unwrap(), false)).collect::<Vec<(u32, bool)>>());
    }

    boards.push(board);
  }

  let mut score = 0;
  for number in numbers {
    if let Some(winner) = mark(&mut boards, number) {
      score = number * score_board(winner);
      break;
    }
  }

  println!("{}", score);

  Ok(())
}

fn mark(boards: &mut Vec<Vec<Vec<(u32, bool)>>>, number: u32) -> Option<&Vec<Vec<(u32, bool)>>> {
  for board in boards {
    let board = mark_board(board, number);
    if board.is_some() {
      return board;
    }
  }

  None
}

fn mark_board(board: &mut Vec<Vec<(u32, bool)>>, number: u32) -> Option<&Vec<Vec<(u32, bool)>>> {
  'mark_loop: for row in board.iter_mut() {
    for (num, marked) in row.iter_mut() {
      if *num == number {
        *marked = true;
        break 'mark_loop;
      }
    }
  }

  if check_board(board) {
    return Some(&*board);
  }

  None
}

fn check_board(board: &Vec<Vec<(u32, bool)>>) -> bool {
  // check rows
  for row in board {
    if row.iter().all(|(_, marked)| *marked) {
      return true;
    }
  }

  // check columns
  for x in 0..5 {
    if (0..5).all(|y| board[y][x].1) {
      return true;
    }
  }

  false
}

fn score_board(board: &Vec<Vec<(u32, bool)>>) -> u32 {
  let mut sum = 0;
  for row in board {
    sum += row.iter().filter(|(_, marked)| !marked).map(|(val, _)| val).sum::<u32>();
  }

  sum
}
