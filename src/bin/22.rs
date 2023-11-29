advent_of_code::solution!(22);

use advent_of_code::util::list::Array2D;

enum Cell {
    Wall,
    Space,
    None,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Rotation {
    Clockwise,
    CounterClockwise,
}

enum Command {
    ChangeDirection(Rotation),
    Move(u32),
}

fn parse_data(input: &str) -> (Vec<Command>, Array2D<Cell>) {
    let line_size = input.lines().next().unwrap().len();
    let mut grid = Array2D::new(line_size);

    let (left_part, right_part) = input.split_once("\n\n").unwrap();

    let none_repeat_iter = std::iter::repeat_with(|| Cell::None);

    for line in left_part.lines() {
        let line_iter = line.chars().map(|x| match x {
            '#' => Cell::Wall,
            '.' => Cell::Space,
            _ => Cell::None,
        });

        grid.add_line(line_iter.chain(none_repeat_iter).take(line_size))
    }

    let mut commands = vec![];
    let mut counter = 0;

    for el in right_part.as_bytes() {
        if el.is_ascii_digit() {
            counter = counter * 10 + (el - b'0') as u32;
        } else {
            commands.push(Command::Move(counter));
            counter = 0;
            let rotation = match el {
                b'R' => Rotation::Clockwise,
                b'L' => Rotation::CounterClockwise,
                _ => unreachable!(),
            };
            commands.push(Command::ChangeDirection(rotation));
        }
    }

    commands.push(Command::Move(counter));

    (commands, grid)
}

const fn next_direction(direction: &Direction, rotation: &Rotation) -> Direction {
    match (direction, rotation) {
        (Direction::Left, Rotation::CounterClockwise) => Direction::Down,
        (Direction::Left, Rotation::Clockwise) => Direction::Up,
        (Direction::Right, Rotation::CounterClockwise) => Direction::Up,
        (Direction::Right, Rotation::Clockwise) => Direction::Down,
        (Direction::Up, Rotation::CounterClockwise) => Direction::Left,
        (Direction::Up, Rotation::Clockwise) => Direction::Right,
        (Direction::Down, Rotation::CounterClockwise) => Direction::Right,
        (Direction::Down, Rotation::Clockwise) => Direction::Left,
    }
}

fn calculate_score(x: usize, y: usize, direction: Direction) -> u32 {
    let facing_score = match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    4 * (x as u32 + 1) + 1000 * (y as u32 + 1) + facing_score
}

fn part_x<F>(commands: &[Command], grid: &Array2D<Cell>, next_state_f: F) -> u32
where
    F: Fn(&Array2D<Cell>, usize, usize, &Direction) -> (usize, usize, Direction),
{
    let (mut x, mut y) = grid
        .iter_keys()
        .find(|&key| matches!(grid[key], Cell::Space))
        .unwrap();
    let mut direction = Direction::Right;

    for command in commands {
        match command {
            Command::ChangeDirection(rotation) => {
                direction = next_direction(&direction, rotation);
            }
            Command::Move(c) => {
                for _ in 0..*c {
                    let (nx, ny, nd) = next_state_f(grid, x, y, &direction);
                    if let Cell::Wall = grid[(nx, ny)] {
                        break;
                    } else {
                        x = nx;
                        y = ny;
                        direction = nd;
                    }
                }
            }
        }
    }

    calculate_score(x, y, direction)
}

fn next_state_part_one(
    grid: &Array2D<Cell>,
    x: usize,
    y: usize,
    direction: &Direction,
) -> (usize, usize, Direction) {
    let mut x = x;
    let mut y = y;

    loop {
        (x, y) = match direction {
            Direction::Up => {
                if y == 0 {
                    (x, grid.len() - 1)
                } else {
                    (x, y - 1)
                }
            }
            Direction::Down => {
                if y == grid.len() - 1 {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
            }
            Direction::Left => {
                if x == 0 {
                    (grid.len_line() - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            Direction::Right => {
                if x == grid.len_line() - 1 {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
        };

        match grid[(x, y)] {
            Cell::None => continue,
            _ => break,
        };
    }

    match direction {
        Direction::Up => (x, y, Direction::Up),
        Direction::Down => (x, y, Direction::Down),
        Direction::Left => (x, y, Direction::Left),
        Direction::Right => (x, y, Direction::Right),
    }
}

fn next_state_part_two(
    _: &Array2D<Cell>,
    x: usize,
    y: usize,
    direction: &Direction,
) -> (usize, usize, Direction) {
    match (x, y, direction) {
        (50, 0..=49, Direction::Left) => (0, 149 - y, Direction::Right),
        (50..=99, 0, Direction::Up) => (0, x + 100, Direction::Right),
        (100..=149, 0, Direction::Up) => (x - 100, 199, Direction::Up),
        (149, 0..=49, Direction::Right) => (99, 149 - y, Direction::Left),
        (100..=149, 49, Direction::Down) => (99, x - 50, Direction::Left),
        (99, 50..=99, Direction::Right) => (y + 50, 49, Direction::Up),
        (50, 50..=99, Direction::Left) => (y - 50, 100, Direction::Down),
        (99, 100..=149, Direction::Right) => (149, 149 - y, Direction::Left),
        (50..=99, 149, Direction::Down) => (49, x + 100, Direction::Left),
        (49, 150..=199, Direction::Right) => (y - 100, 149, Direction::Up),
        (0..=49, 199, Direction::Down) => (x + 100, 0, Direction::Down),
        (0, 150..=199, Direction::Left) => (y - 100, 0, Direction::Down),
        (0, 100..=149, Direction::Left) => (50, 149 - y, Direction::Right),
        (0..=49, 100, Direction::Up) => (50, x + 50, Direction::Right),
        (_, _, Direction::Up) => (x, y - 1, Direction::Up),
        (_, _, Direction::Down) => (x, y + 1, Direction::Down),
        (_, _, Direction::Left) => (x - 1, y, Direction::Left),
        (_, _, Direction::Right) => (x + 1, y, Direction::Right),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (commands, grid) = parse_data(input);

    let result = part_x(&commands, &grid, next_state_part_one);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (commands, grid) = parse_data(input);

    let result = part_x(&commands, &grid, next_state_part_two);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(135107));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(27279));
    }
}
