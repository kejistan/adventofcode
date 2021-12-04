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
    let is_final_board = boards.len() == 1;
    mark(&mut boards, number);

    if is_final_board && check_board(&boards[0]) {
      score = number * score_board(&boards[0]);
      break;
    } else {
      boards = boards.into_iter().filter(|board| !check_board(board)).collect();
    }
  }

  println!("{}", score);

  Ok(())
}

fn mark(boards: &mut Vec<Vec<Vec<(u32, bool)>>>, number: u32) {
  for board in boards {
    mark_board(board, number);
  }
}

fn mark_board(board: &mut Vec<Vec<(u32, bool)>>, number: u32) {
  'mark_loop: for row in board.iter_mut() {
    for (num, marked) in row.iter_mut() {
      if *num == number {
        *marked = true;
        break 'mark_loop;
      }
    }
  }
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
