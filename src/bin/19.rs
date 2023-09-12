use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use regex::Regex;

struct Blueprint {
    id: u32,
    ore_robot_cost_ore: u32,
    clay_robot_cost_ore: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_obsidian: u32,
    max_ore_cost: u32,
    max_clay_cost: u32,
    max_obsidian_cost: u32,
}

fn parse_data(input: &str) -> Vec<Blueprint> {
    let mut regex_string = String::new();
    regex_string += r"^Blueprint (\d+): ";
    regex_string += r"Each ore robot costs (\d+) ore. ";
    regex_string += r"Each clay robot costs (\d+) ore. ";
    regex_string += r"Each obsidian robot costs (\d+) ore and (\d+) clay. ";
    regex_string += r"Each geode robot costs (\d+) ore and (\d+) obsidian.$";

    let re = Regex::new(&regex_string).unwrap();

    input
        .lines()
        .map(|x| {
            let captures = re.captures(x).unwrap();

            let id = captures[1].parse().unwrap();
            let ore_robot_cost_ore = captures[2].parse().unwrap();
            let clay_robot_cost_ore = captures[3].parse().unwrap();
            let obsidian_robot_cost_ore = captures[4].parse().unwrap();
            let obsidian_robot_cost_clay = captures[5].parse().unwrap();
            let geode_robot_cost_ore = captures[6].parse().unwrap();
            let geode_robot_cost_obsidian = captures[7].parse().unwrap();
            let max_obsidian_cost = geode_robot_cost_obsidian;
            let max_clay_cost = obsidian_robot_cost_clay;
            let max_ore_cost = [
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obsidian_robot_cost_ore,
                geode_robot_cost_ore,
            ]
            .into_iter()
            .max()
            .unwrap();

            Blueprint {
                id,
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obsidian_robot_cost_ore,
                obsidian_robot_cost_clay,
                geode_robot_cost_ore,
                geode_robot_cost_obsidian,
                max_ore_cost,
                max_clay_cost,
                max_obsidian_cost,
            }
        })
        .collect()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn get_next_states(blueprint: &Blueprint, state: &State) -> Vec<State> {
    if state.ore >= blueprint.geode_robot_cost_ore
        && state.obsidian >= blueprint.geode_robot_cost_obsidian
    {
        return vec![State {
            time: state.time + 1,
            ore: state.ore + state.ore_robots - blueprint.geode_robot_cost_ore,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots - blueprint.geode_robot_cost_obsidian,
            geode: state.geode + state.geode_robots,
            ore_robots: state.ore_robots,
            clay_robots: state.clay_robots,
            obsidian_robots: state.obsidian_robots,
            geode_robots: state.geode_robots + 1,
        }];
    }

    if state.obsidian_robots < blueprint.max_obsidian_cost
        && state.ore >= blueprint.obsidian_robot_cost_ore
        && state.clay >= blueprint.obsidian_robot_cost_clay
    {
        return vec![State {
            time: state.time + 1,
            ore: state.ore + state.ore_robots - blueprint.obsidian_robot_cost_ore,
            clay: state.clay + state.clay_robots - blueprint.obsidian_robot_cost_clay,
            obsidian: state.obsidian + state.obsidian_robots,
            geode: state.geode + state.geode_robots,
            ore_robots: state.ore_robots,
            clay_robots: state.clay_robots,
            obsidian_robots: state.obsidian_robots + 1,
            geode_robots: state.geode_robots,
        }];
    }

    let mut result = Vec::with_capacity(3);

    if state.clay_robots < blueprint.max_clay_cost && state.ore >= blueprint.clay_robot_cost_ore {
        result.push(State {
            time: state.time + 1,
            ore: state.ore + state.ore_robots - blueprint.clay_robot_cost_ore,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            geode: state.geode + state.geode_robots,
            ore_robots: state.ore_robots,
            clay_robots: state.clay_robots + 1,
            obsidian_robots: state.obsidian_robots,
            geode_robots: state.geode_robots,
        });
    }

    if state.ore_robots < blueprint.max_ore_cost && state.ore >= blueprint.ore_robot_cost_ore {
        result.push(State {
            time: state.time + 1,
            ore: state.ore + state.ore_robots - blueprint.ore_robot_cost_ore,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            geode: state.geode + state.geode_robots,
            ore_robots: state.ore_robots + 1,
            clay_robots: state.clay_robots,
            obsidian_robots: state.obsidian_robots,
            geode_robots: state.geode_robots,
        });
    }

    result.push(State {
        time: state.time + 1,
        ore: state.ore + state.ore_robots,
        clay: state.clay + state.clay_robots,
        obsidian: state.obsidian + state.obsidian_robots,
        geode: state.geode + state.geode_robots,
        ore_robots: state.ore_robots,
        clay_robots: state.clay_robots,
        obsidian_robots: state.obsidian_robots,
        geode_robots: state.geode_robots,
    });

    result
}

fn sum_natural_numbers(a: i32, d: i32, n: i32) -> i32 {
    n * (2 * a + (n - 1) * d) / 2
}

fn can_be_discarded(
    state: &State,
    best_result: &State,
    blueprint: &Blueprint,
    max_time: u32,
) -> bool {
    let diff_time = max_time as i32 - state.time as i32;

    let calculated_state_clay =
        state.clay as i32 + sum_natural_numbers(state.clay_robots as i32, 1, diff_time);

    let max_additional_obsidian_robots = i32::min(
        calculated_state_clay / blueprint.obsidian_robot_cost_clay as i32,
        diff_time,
    );
    let calculated_state_obsidian = state.obsidian as i32
        + sum_natural_numbers(
            state.obsidian_robots as i32,
            1,
            max_additional_obsidian_robots,
        )
        + (diff_time - max_additional_obsidian_robots)
            * (state.obsidian_robots as i32 + max_additional_obsidian_robots);

    let max_additional_geode_robots = i32::min(
        calculated_state_obsidian / blueprint.geode_robot_cost_obsidian as i32,
        diff_time,
    );
    let calculated_state_geode = state.geode as i32
        + sum_natural_numbers(state.geode_robots as i32, 1, max_additional_geode_robots)
        + (diff_time - max_additional_geode_robots)
            * (state.geode_robots as i32 + max_additional_geode_robots);

    calculated_state_geode <= best_result.geode as i32
}

struct DFS<'a> {
    init_state: State,
    blueprint: &'a Blueprint,
    max_time: u32,
    best_result: State,
    discovered: HashSet<State>,
}

impl<'a> DFS<'a> {
    fn find_best(&mut self) -> State {
        let mut queue = VecDeque::new();
        queue.push_front(self.init_state);

        while let Some(current_state) = queue.pop_front() {
            self.discovered.insert(current_state);

            for n in get_next_states(self.blueprint, &current_state)
                .into_iter()
                .rev()
            {
                if self.discovered.contains(&n) {
                    continue;
                }

                if n.time == self.max_time {
                    if self.best_result.geode < n.geode {
                        self.best_result = n;
                    }
                    continue;
                }

                if can_be_discarded(&n, &self.best_result, self.blueprint, self.max_time) {
                    continue;
                }

                queue.push_front(n);
            }
        }

        self.best_result
    }
}

fn find_max_geodes<const MAX_TIME: u32>(blueprint: &Blueprint) -> u32 {
    let mut dfs = DFS {
        init_state: State {
            ore_robots: 1,
            ..Default::default()
        },
        blueprint,
        max_time: MAX_TIME,
        best_result: State::default(),
        discovered: HashSet::new(),
    };

    dfs.find_best().geode as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.iter().map(|b| b.id * find_max_geodes::<24>(b)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .iter()
        .take(3)
        .map(|b| find_max_geodes::<32>(b))
        .product();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(23));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(29348));
    }
}
