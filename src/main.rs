use std::collections::{HashMap, HashSet};
use std::{io};
use std::io::{BufReader, BufRead};

use regex::Regex;

mod coordinate;

#[derive(Debug)]
struct Valve {
  id: String,
  flow_rate: u32,
  connections: Vec<String>,
}

#[derive(Clone)]
struct ExplorationState<'a> {
  open_valves: HashSet<String>,
  current_pos: &'a str,
  score: u32,
  remaining_time: u32,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let valve_info_regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? ").unwrap();
  let tunnels_regex = Regex::new(r"[A-Z]+").unwrap();

  let valves = input.lines().map(|result| {
    let line = result.unwrap();
    let captures = valve_info_regex.captures(&line).unwrap();
    let id = captures.get(1).unwrap().as_str().to_string();
    let flow_rate = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();

    let connections = tunnels_regex.find_iter(&line[captures.get(0).unwrap().end()..]).map(|cap| cap.as_str().to_string()).collect::<Vec<String>>();

    (id.clone(), Valve {
      id,
      flow_rate,
      connections,
    })
  }).collect::<HashMap<String, Valve>>();

  let first_state = ExplorationState { open_valves: HashSet::new(), current_pos: "AA", score: 0, remaining_time: 30 };
  let mut states = Vec::new();
  states.push(first_state);

  for _ in 0..30 {
    let mut new_states = Vec::new();
    let max_score = states.iter().map(|state| state.score).max().unwrap();
    for mut state in states.into_iter().filter(|state| state.score + 40 >= max_score) {
      let score = score_valves(&valves, &state.open_valves);
      state.remaining_time -= 1;
      state.score += score;

      if !state.open_valves.contains(state.current_pos) {
        let mut new_state = state.clone();
        new_state.open_valves.insert(state.current_pos.to_string());
        new_states.push(new_state);
      }

      let current_valve = valves.get(state.current_pos).unwrap();
      for neighbor in current_valve.connections.iter() {
        let mut new_state = state.clone();
        new_state.current_pos = neighbor;
        new_states.push(new_state);
      }
    }
    states = new_states;
  }

  let result = states.into_iter().map(|state| state.score).max().unwrap();

  println!("{}", result);

  Ok(())
}

fn score_valves(valves: &HashMap<String, Valve>, open_valves: &HashSet<String>) -> u32 {
  valves.values().filter_map(|valve| {
    if open_valves.contains(&valve.id) {
      Some(valve.flow_rate)
    } else {
      None
    }
  }).sum()
}
