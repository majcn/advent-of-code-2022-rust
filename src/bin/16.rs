use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

type ValveName = [u8; 2];

struct Valve {
    name: ValveName,
    flow_rate: u32,
    tunnels: Vec<ValveName>,
}

struct ValveArray {
    raw_data: HashMap<ValveName, Valve>,
}

fn parse_data(input: &str) -> ValveArray {
    let re_str: &str = r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z][A-Z](?:, )?)+)$";
    let re = Regex::new(re_str).unwrap();

    fn str_to_valve_name(s: &str) -> ValveName {
        let mut iter = s.as_bytes().iter();
        [*iter.next().unwrap(), *iter.next().unwrap()]
    }

    let valves = input
        .lines()
        .map(|x| {
            let captures = re.captures(x).unwrap();
            let (_, [name, flow_rate, tunnel_names]) = captures.extract();
            Valve {
                name: str_to_valve_name(name),
                flow_rate: flow_rate.parse().unwrap(),
                tunnels: tunnel_names.split(", ").map(str_to_valve_name).collect(),
            }
        })
        .collect::<Vec<_>>();

    let mut result = HashMap::with_capacity(valves.len());
    for valve in valves {
        result.insert(valve.name, valve);
    }

    ValveArray { raw_data: result }
}

fn bfs(data: &ValveArray, start_node_name: ValveName, end_node_name: ValveName) -> Option<u32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start_node_name, 0));
    visited.insert(start_node_name);

    while let Some((node_name, time)) = queue.pop_front() {
        if node_name == end_node_name {
            return Some(time);
        }

        for neighbor_name in data.raw_data[&node_name].tunnels.iter() {
            if !visited.contains(neighbor_name) {
                queue.push_back((*neighbor_name, time + 1));
                visited.insert(*neighbor_name);
            }
        }
    }

    None
}

struct State {
    time: u32,
    location: ValveName,
    pressure_released: u32,
    opened_valves: HashSet<ValveName>,
}

fn find_all_final_states<F>(init_state: State, next_states_f: F) -> Vec<State>
where
    F: Fn(&State) -> Vec<State>,
{
    let mut candidates: Vec<State> = vec![];

    // TODO: dejansko ne znam napisat rekurzije za to v rustu...
    let mut queue = vec![init_state];
    while let Some(state) = queue.pop() {
        queue.extend(next_states_f(&state));
        candidates.push(state);
    }

    candidates
}

fn get_next_states(
    data: &ValveArray,
    state: &State,
    valve_paths: &HashMap<ValveName, Vec<(ValveName, u32)>>,
    max_time: u32,
) -> Vec<State> {
    let mut result = Vec::with_capacity(valve_paths[&state.location].len());

    for valve_path in &valve_paths[&state.location] {
        if state.opened_valves.contains(&valve_path.0) {
            continue;
        }

        let new_time = state.time + valve_path.1;
        if new_time > max_time {
            continue;
        }

        let new_pressure_released = state.pressure_released
            + (max_time - new_time) * data.raw_data[&valve_path.0].flow_rate;

        let mut new_opened_valves = HashSet::with_capacity(state.opened_valves.len() + 1);
        new_opened_valves.extend(state.opened_valves.iter());
        new_opened_valves.insert(valve_path.0);

        result.push(State {
            time: state.time + valve_path.1,
            location: valve_path.0,
            pressure_released: new_pressure_released,
            opened_valves: new_opened_valves,
        });
    }

    result
}

fn part_x<const T: u32>(data: &ValveArray) -> Vec<State> {
    let mut valve_paths: HashMap<ValveName, Vec<(ValveName, u32)>> = HashMap::new();

    for (i, v1) in data.raw_data.values().enumerate() {
        for (j, v2) in data.raw_data.values().enumerate() {
            if i != j && v2.flow_rate > 0 {
                let p = valve_paths.entry(v1.name).or_insert(vec![]);
                p.push((v2.name, bfs(data, v1.name, v2.name).unwrap() + 1));
            }
        }
    }

    let init_state = State {
        time: 0,
        location: [b'A', b'A'],
        pressure_released: 0,
        opened_valves: HashSet::new(),
    };

    find_all_final_states(init_state, |state| {
        get_next_states(data, state, &valve_paths, T)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut all_final_states = part_x::<30>(&data);
    all_final_states.sort_unstable_by_key(|x| x.pressure_released);

    let result = all_final_states
        .iter()
        .rev()
        .map(|x| x.pressure_released)
        .next();

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut all_final_states = part_x::<26>(&data);
    all_final_states.sort_unstable_by_key(|x| x.pressure_released);

    let mut result = 0;
    for s1 in all_final_states.iter().rev() {
        for s2 in all_final_states.iter().rev() {
            if result >= s1.pressure_released + s2.pressure_released {
                break;
            }

            if s1.opened_valves.is_disjoint(&s2.opened_valves) {
                result = u32::max(result, s1.pressure_released + s2.pressure_released);
            }
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
