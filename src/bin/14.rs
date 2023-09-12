use std::collections::HashSet;

use advent_of_code::util::point::Point;

fn parse_data(input: &str) -> Vec<Vec<Point>> {
    fn parse_line(line: &str) -> Vec<Point> {
        line.split(" -> ")
            .map(|p| p.split_once(',').unwrap())
            .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
            .collect()
    }

    input.lines().map(parse_line).collect()
}

fn build_grid(data: Vec<Vec<Point>>) -> HashSet<Point> {
    let mut grid = HashSet::new();
    for path in data {
        for w in path.windows(2) {
            let start = w[0]; // TODO: to se zihr da narest direkt v for loopu
            let end = w[1]; // TODO: to se zihr da narest direkt v for loopu
            let start_x = i32::min(start.x, end.x);
            let end_x = i32::max(start.x, end.x);
            let start_y = i32::min(start.y, end.y);
            let end_y = i32::max(start.y, end.y);
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    grid.insert(Point::new(x, y));
                }
            }
        }
    }
    grid
}

fn part_x<F>(data: Vec<Vec<Point>>, end_predicate: F) -> u32
where
    F: Fn(Point, i32) -> bool,
{
    let mut grid = build_grid(data);

    let max_y = grid.iter().map(|p| p.y).max().unwrap();

    let location_down = Point::new(0, 1);
    let location_down_left = Point::new(-1, 1);
    let location_down_right = Point::new(1, 1);

    let mut i = 0;
    loop {
        let mut sand = Point::new(500, 0);
        while sand.y != max_y + 1 {
            let mut next_sand = sand + location_down;
            if grid.contains(&next_sand) {
                next_sand = sand + location_down_left;
            }
            if grid.contains(&next_sand) {
                next_sand = sand + location_down_right;
            }
            if grid.contains(&next_sand) {
                break;
            }
            sand = next_sand;
        }

        if end_predicate(sand, max_y) {
            return i;
        }

        grid.insert(sand);
        i += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data, |sand, max_y| sand.y == max_y + 1);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let goal = Point::new(500, 0);
    let result = part_x(data, |sand, _| sand == goal) + 1;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
