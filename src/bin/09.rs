use std::collections::HashSet;

use advent_of_code::util::point::{Point, DOWN, LEFT, RIGHT, UP};

struct Command {
    direction: u8,
    steps: u32,
}

fn parse_data(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| Command {
            direction: x.0.as_bytes()[0],
            steps: x.1.parse().unwrap(),
        })
        .collect()
}

fn part_x<const N: usize>(data: &[Command]) -> u32 {
    let mut rope = vec![Point { x: 0, y: 0 }; N];

    let mut visited = HashSet::new();
    visited.insert(Point { x: 0, y: 0 });
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
                let head = rope[(i - 1) as usize];
                let tail = &mut rope[i as usize];

                if i32::abs(head.x - tail.x) > 1 || i32::abs(head.y - tail.y) > 1 {
                    let diff_x = if head.x < tail.x {
                        -1
                    } else if head.x > tail.x {
                        1
                    } else {
                        0
                    };

                    let diff_y = if head.y < tail.y {
                        -1
                    } else if head.y > tail.y {
                        1
                    } else {
                        0
                    };

                    let diff_point = Point {
                        x: diff_x,
                        y: diff_y,
                    };

                    *tail += diff_point;
                }
            }

            visited.insert(rope.iter().last().copied().unwrap());
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
