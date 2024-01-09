advent_of_code::solution!(23);

use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::majcn::point::Point;

enum CardinalDirection {
    North,
    South,
    East,
    West,
}

fn parse_data(input: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            if v == '#' {
                result.insert(Point::new(x as i32, y as i32));
            }
        }
    }
    result
}

const NORTH: CardinalDirection = CardinalDirection::North;
const SOUTH: CardinalDirection = CardinalDirection::South;
const WEST: CardinalDirection = CardinalDirection::West;
const EAST: CardinalDirection = CardinalDirection::East;
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
    NEIGHBORS_8
        .into_iter()
        .map(|x| elf + x)
        .find(|x| elfs.contains(x))?;

    for priority in PRIORITIES[priorities_index].iter() {
        match priority {
            CardinalDirection::North => {
                if (-1..=1).all(|x| !elfs.contains(&(elf + Point::new(x, -1)))) {
                    return Some(elf + Point::new(0, -1));
                }
            }
            CardinalDirection::South => {
                if (-1..=1).all(|x| !elfs.contains(&(elf + Point::new(x, 1)))) {
                    return Some(elf + Point::new(0, 1));
                }
            }
            CardinalDirection::East => {
                if (-1..=1).all(|y| !elfs.contains(&(elf + Point::new(1, y)))) {
                    return Some(elf + Point::new(1, 0));
                }
            }
            CardinalDirection::West => {
                if (-1..=1).all(|y| !elfs.contains(&(elf + Point::new(-1, y)))) {
                    return Some(elf + Point::new(-1, 0));
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

                if next_locations.contains_key(&next_elf_location) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(110));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
