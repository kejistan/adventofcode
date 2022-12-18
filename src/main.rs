use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::{io};
use std::io::{BufReader, BufRead};

use regex::Regex;

mod coordinate;

#[derive(Debug)]
struct Valve {
  id: String,
  flow_rate: u32,
  connections: Vec<String>,
  distances: HashMap<String, u32>,
}

#[derive(Clone, Debug)]
struct ExplorationState {
  targets: Vec<String>,
  current_pos: String,
  elephant_current_pos: String,
  score: u32,
  remaining_time: u32,
  elephant_remaining_time: u32,
}

fn main() -> io::Result<()> {
  let input = BufReader::new(io::stdin());
  let valve_info_regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? ").unwrap();
  let tunnels_regex = Regex::new(r"[A-Z]+").unwrap();

  let mut valves = input.lines().map(|result| {
    let line = result.unwrap();
    let captures = valve_info_regex.captures(&line).unwrap();
    let id = captures.get(1).unwrap().as_str().to_string();
    let flow_rate = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();

    let connections = tunnels_regex.find_iter(&line[captures.get(0).unwrap().end()..]).map(|cap| cap.as_str().to_string()).collect::<Vec<String>>();

    (id.clone(), Valve {
      id,
      flow_rate,
      connections,
      distances: HashMap::new(),
    })
  }).collect::<HashMap<String, Valve>>();

  let valves_with_any_value = valves.values().filter_map(|valve| {
    if valve.flow_rate > 0 {
      Some(valve.id.clone())
    } else {
      None
    }
  }).collect::<Vec<String>>();

  set_distances(&mut valves, &valves_with_any_value);

  let mut max_score = 0;
  let mut exploration_queue = VecDeque::new();
  exploration_queue.push_back(ExplorationState {
    targets: valves_with_any_value,
    current_pos: "AA".to_string(),
    elephant_current_pos: "AA".to_string(),
    score: 0,
    remaining_time: 26,
    elephant_remaining_time: 26,
  });

  let mut count = 0;
  while let Some(exploration) = exploration_queue.pop_front() {
    if exploration.targets.len() == 0 {
      max_score = max(exploration.score, max_score);
      continue;
    }

    let scored_targets = score_targets(&exploration.targets, &valves, &exploration.current_pos, exploration.remaining_time);
    let elephant_scored_targets = score_targets(&exploration.targets, &valves, &exploration.elephant_current_pos, exploration.elephant_remaining_time);

    let mut max_target_scores: HashMap<&str, u32> = HashMap::with_capacity(scored_targets.len());
    for (target, score, _) in scored_targets.iter() {
      max_target_scores.insert(*target, *score);
    }
    for (target, score, _) in elephant_scored_targets.iter() {
      let max_score = max_target_scores.get_mut(target.as_str()).unwrap();
      *max_score = max(*score, *max_score);
    }

    let this_explorations_maximum = exploration.score + max_target_scores.values().sum::<u32>();
    if max_score >= this_explorations_maximum {
      // No possible path in this state will exceed the current maximum score
      continue;
    }

    count += 1;

    // Elephant moves
    if exploration.elephant_remaining_time >= exploration.remaining_time {
      let mut next_states = elephant_scored_targets.into_iter().map(|(target, score, elephant_remaining_time)| {
        let targets = exploration.targets.iter().filter(|t| *t != target).cloned().collect::<Vec<String>>();
  
        ExplorationState {
          current_pos: exploration.current_pos.clone(),
          elephant_current_pos: target.clone(),
          targets,
          score: exploration.score + score,
          remaining_time: exploration.remaining_time,
          elephant_remaining_time,
        }
      });
  
      // Take the highest priority target for a depth first traversal
      if let Some(state) = next_states.next() {
        exploration_queue.push_front(state);
      }
  
      // Push the remaining targets to the end of the queue for a breadth first traversal
      while let Some(state) = next_states.next() {
        exploration_queue.push_back(state);
      }
    }

    // Non-elephant moves
    if exploration.elephant_remaining_time <= exploration.remaining_time {
      let mut next_states = scored_targets.into_iter().map(|(target, score, remaining_time)| {
        let targets = exploration.targets.iter().filter(|t| *t != target).cloned().collect::<Vec<String>>();
  
        ExplorationState {
          current_pos: target.clone(),
          elephant_current_pos: exploration.elephant_current_pos.clone(),
          targets,
          score: exploration.score + score,
          remaining_time,
          elephant_remaining_time: exploration.elephant_remaining_time,
        }
      });
  
      // Take the highest priority target for a depth first traversal
      if let Some(state) = next_states.next() {
        exploration_queue.push_front(state);
      }
  
      // Push the remaining targets to the end of the queue for a breadth first traversal
      while let Some(state) = next_states.next() {
        exploration_queue.push_back(state);
      }
    }
  }

  println!("found in {}", count);

  let result = max_score;

  println!("{}", result);

  Ok(())
}

fn score_targets<'a>(targets: &'a [String], valves: &HashMap<String, Valve>, current: &String, remaining_time: u32) -> Vec<(&'a String, u32, u32)> {
  let current_valve = valves.get(current).unwrap();
  let mut scored_targets = targets.iter().map(|t| {
    let distance = current_valve.distances.get(t).unwrap();
    let score;
    let time;
    if distance + 1 < remaining_time {
      let target = valves.get(t).unwrap();
      time = remaining_time - distance - 1;
      score = time * target.flow_rate;
    } else {
      time = 0;
      score = 0;
    }

    (t, score, time)
  }).collect::<Vec<_>>();

  // order in descending order. Maximum score first
  scored_targets.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
  scored_targets
}

fn set_distances(valves: &mut HashMap<String, Valve>, targets: &[String]) {
  for id in targets.iter() {
    let mut queue = VecDeque::new();
    queue.push_back((id.clone(), 0));

    while let Some((target, distance)) = queue.pop_front() {
      let target_valve = &mut valves.get_mut(&target).unwrap();
      let distances = &mut target_valve.distances;
      if !distances.contains_key(id) {
        distances.insert(id.to_string(), distance);
        queue.extend(target_valve.connections.iter().map(|id| (id.clone(), distance + 1)));
      }
    }
  }
}
