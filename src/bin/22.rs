use advent_of_code::util::list::Array2D;

enum Cell {
    WALL,
    SPACE,
    NONE,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

enum Rotation {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

enum Command {
    ChangeDirection(Rotation),
    Move(u32),
}

fn parse_data(input: &str) -> (Vec<Command>, Array2D<Cell>) {
    let line_size = input.lines().next().unwrap().len();
    let mut grid = Array2D::new(line_size);

    let (left_part, right_part) = input.split_once("\n\n").unwrap();

    let none_repeat_iter = std::iter::repeat_with(|| Cell::NONE);

    for line in left_part.lines() {
        let line_iter = line.as_bytes().into_iter().map(|x| match x {
            b'#' => Cell::WALL,
            b'.' => Cell::SPACE,
            _ => Cell::NONE,
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
                b'R' => Rotation::CLOCKWISE,
                b'L' => Rotation::COUNTERCLOCKWISE,
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
        (Direction::LEFT, Rotation::COUNTERCLOCKWISE) => Direction::DOWN,
        (Direction::LEFT, Rotation::CLOCKWISE) => Direction::UP,
        (Direction::RIGHT, Rotation::COUNTERCLOCKWISE) => Direction::UP,
        (Direction::RIGHT, Rotation::CLOCKWISE) => Direction::DOWN,
        (Direction::UP, Rotation::COUNTERCLOCKWISE) => Direction::LEFT,
        (Direction::UP, Rotation::CLOCKWISE) => Direction::RIGHT,
        (Direction::DOWN, Rotation::COUNTERCLOCKWISE) => Direction::RIGHT,
        (Direction::DOWN, Rotation::CLOCKWISE) => Direction::LEFT,
    }
}

fn calculate_score(x: usize, y: usize, direction: Direction) -> u32 {
    let facing_score = match direction {
        Direction::RIGHT => 0,
        Direction::DOWN => 1,
        Direction::LEFT => 2,
        Direction::UP => 3,
    };

    4 * (x as u32 + 1) + 1000 * (y as u32 + 1) + facing_score
}

fn part_x<F>(commands: &[Command], grid: &Array2D<Cell>, next_state_f: F) -> u32
where
    F: Fn(&Array2D<Cell>, usize, usize, &Direction) -> (usize, usize, Direction),
{
    let (mut x, mut y) = grid
        .iter_keys()
        .find(|&key| match grid[key] {
            Cell::SPACE => true,
            _ => false,
        })
        .unwrap();
    let mut direction = Direction::RIGHT;

    for command in commands {
        match command {
            Command::ChangeDirection(rotation) => {
                direction = next_direction(&direction, rotation);
            }
            Command::Move(c) => {
                for _ in 0..*c {
                    let (nx, ny, nd) = next_state_f(grid, x, y, &direction);
                    if let Cell::WALL = grid[(nx, ny)] {
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
            Direction::UP => {
                if y == 0 {
                    (x, grid.len() - 1)
                } else {
                    (x, y - 1)
                }
            }
            Direction::DOWN => {
                if y == grid.len() - 1 {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
            }
            Direction::LEFT => {
                if x == 0 {
                    (grid.len_line() - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            Direction::RIGHT => {
                if x == grid.len_line() - 1 {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
        };

        match grid[(x, y)] {
            Cell::NONE => continue,
            _ => break,
        };
    }

    match direction {
        Direction::UP => (x, y, Direction::UP),
        Direction::DOWN => (x, y, Direction::DOWN),
        Direction::LEFT => (x, y, Direction::LEFT),
        Direction::RIGHT => (x, y, Direction::RIGHT),
    }
}

#[allow(unused_comparisons)]
fn next_state_part_two(
    _: &Array2D<Cell>,
    x: usize,
    y: usize,
    direction: &Direction,
) -> (usize, usize, Direction) {
    match (x, y, direction) {
        (50, 0..=49, Direction::LEFT) => (0, 149 - y, Direction::RIGHT),
        (50..=99, 0, Direction::UP) => (0, x + 100, Direction::RIGHT),
        (100..=149, 0, Direction::UP) => (x - 100, 199, Direction::UP),
        (149, 0..=49, Direction::RIGHT) => (99, 149 - y, Direction::LEFT),
        (100..=149, 49, Direction::DOWN) => (99, x - 50, Direction::LEFT),
        (99, 50..=99, Direction::RIGHT) => (y + 50, 49, Direction::UP),
        (50, 50..=99, Direction::LEFT) => (y - 50, 100, Direction::DOWN),
        (99, 100..=149, Direction::RIGHT) => (149, 149 - y, Direction::LEFT),
        (50..=99, 149, Direction::DOWN) => (49, x + 100, Direction::LEFT),
        (49, 150..=199, Direction::RIGHT) => (y - 100, 149, Direction::UP),
        (0..=49, 199, Direction::DOWN) => (x + 100, 0, Direction::DOWN),
        (0, 150..=199, Direction::LEFT) => (y - 100, 0, Direction::DOWN),
        (0, 100..=149, Direction::LEFT) => (50, 149 - y, Direction::RIGHT),
        (0..=49, 100, Direction::UP) => (50, x + 50, Direction::RIGHT),
        (_, _, Direction::UP) => (x, y - 1, Direction::UP),
        (_, _, Direction::DOWN) => (x, y + 1, Direction::DOWN),
        (_, _, Direction::LEFT) => (x - 1, y, Direction::LEFT),
        (_, _, Direction::RIGHT) => (x + 1, y, Direction::RIGHT),
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

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(135107));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(27279));
    }
}
