use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::util::point::Point;

enum CardinalDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl From<u8> for CardinalDirection {
    fn from(value: u8) -> Self {
        match value {
            b'N' => CardinalDirection::NORTH,
            b'S' => CardinalDirection::SOUTH,
            b'E' => CardinalDirection::EAST,
            b'W' => CardinalDirection::WEST,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&u8> for CardinalDirection {
    type Error = bool;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'N' => Ok(CardinalDirection::NORTH),
            b'S' => Ok(CardinalDirection::SOUTH),
            b'E' => Ok(CardinalDirection::EAST),
            b'W' => Ok(CardinalDirection::WEST),
            _ => Err(true),
        }
    }
}

fn parse_data(input: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.as_bytes().iter().enumerate() {
            if v == &b'#' {
                result.insert(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    result
}

const NORTH: CardinalDirection = CardinalDirection::NORTH;
const SOUTH: CardinalDirection = CardinalDirection::SOUTH;
const WEST: CardinalDirection = CardinalDirection::WEST;
const EAST: CardinalDirection = CardinalDirection::EAST;
const PRIORITIES: [[CardinalDirection; 4]; 4] = [
    [NORTH, SOUTH, WEST, EAST],
    [SOUTH, WEST, EAST, NORTH],
    [WEST, EAST, NORTH, SOUTH],
    [EAST, NORTH, SOUTH, WEST],
];

const NEIGHBORS_8: [Point; 8] = [
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: -1 },
];

fn get_next_location(elfs: &HashSet<Point>, elf: Point, priorities_index: usize) -> Option<Point> {
    let neighbor = NEIGHBORS_8
        .into_iter()
        .map(|x| elf + x)
        .find(|x| elfs.contains(x));

    if let None = neighbor {
        return None;
    }

    for priority in PRIORITIES[priorities_index].iter() {
        match priority {
            CardinalDirection::NORTH => {
                if (-1..=1).all(|x| !elfs.contains(&(elf + Point { x, y: -1 }))) {
                    return Some(elf + Point { x: 0, y: -1 });
                }
            }
            CardinalDirection::SOUTH => {
                if (-1..=1).all(|x| !elfs.contains(&(elf + Point { x, y: 1 }))) {
                    return Some(elf + Point { x: 0, y: 1 });
                }
            }
            CardinalDirection::EAST => {
                if (-1..=1).all(|y| !elfs.contains(&(elf + Point { x: 1, y }))) {
                    return Some(elf + Point { x: 1, y: 0 });
                }
            }
            CardinalDirection::WEST => {
                if (-1..=1).all(|y| !elfs.contains(&(elf + Point { x: -1, y }))) {
                    return Some(elf + Point { x: -1, y: 0 });
                }
            }
        }
    }

    None
}

fn part_x<F>(elfs: &HashSet<Point>, goal: F) -> (u32, HashSet<Point>)
where
    F: Fn(usize, bool) -> bool,
{
    let mut elfs = elfs.clone();
    let mut i = 0;
    loop {
        let mut blocked_locations_set = HashSet::new();
        let mut next_locations = HashMap::new();
        for elf in elfs.iter().copied() {
            if let Some(next_elf_location) = get_next_location(&elfs, elf, i % 4) {
                if blocked_locations_set.contains(&next_elf_location) {
                    continue;
                }

                if let Some(_) = next_locations.get(&next_elf_location) {
                    next_locations.remove(&next_elf_location);
                    blocked_locations_set.insert(next_elf_location);
                    continue;
                }

                next_locations.insert(next_elf_location, elf);
            }
        }

        let next_locations_len = next_locations.len();

        for (next_location, prev_location) in next_locations {
            elfs.remove(&prev_location);
            elfs.insert(next_location);
        }

        if goal(i + 1, next_locations_len == 0) {
            return (i as u32 + 1, elfs);
        }

        i += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let (_, elfs) = part_x(&data, |i, _| i == 10);

    let minx = elfs.iter().map(|x| x.x).min().unwrap();
    let maxx = elfs.iter().map(|x| x.x).max().unwrap();
    let miny = elfs.iter().map(|x| x.y).min().unwrap();
    let maxy = elfs.iter().map(|x| x.y).max().unwrap();

    let result = (maxy - miny + 1) as u32 * (maxx - minx + 1) as u32 - elfs.len() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let (result, _) = part_x(&data, |_, nothing_happens| nothing_happens);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
