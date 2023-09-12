use std::collections::HashSet;

use advent_of_code::util::point::Point;
use advent_of_code::util::point::DOWN;
use advent_of_code::util::point::LEFT;
use advent_of_code::util::point::RIGHT;
use advent_of_code::util::point::UP;

struct Command {
    direction: u8,
    steps: u32,
}

fn parse_data(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|x| Command {
            direction: x.0.as_bytes()[0],
            steps: x.1.parse().unwrap(),
        })
        .collect()
}

fn part_x<const N: usize>(data: &[Command]) -> u32 {
    let mut rope = vec![Point::new(0, 0); N];

    let mut visited = HashSet::new();
    visited.insert(Point::new(0, 0));
    for command in data {
        for _ in 0..command.steps {
            match command.direction {
                b'R' => rope[0] += RIGHT,
                b'D' => rope[0] += DOWN,
                b'L' => rope[0] += LEFT,
                b'U' => rope[0] += UP,
                _ => {}
            }

            for i in 1..N {
                let head = rope[i - 1];
                let tail = &mut rope[i];

                if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
                    let diff_x = match Ord::cmp(&head.x, &tail.x) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Greater => 1,
                        std::cmp::Ordering::Equal => 0,
                    };

                    let diff_y = match Ord::cmp(&head.y, &tail.y) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Greater => 1,
                        std::cmp::Ordering::Equal => 0,
                    };

                    *tail += Point::new(diff_x, diff_y);
                }
            }

            visited.insert(rope.last().copied().unwrap());
        }
    }

    visited.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<2>(&data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<10>(&data);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
