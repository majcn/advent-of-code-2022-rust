advent_of_code::solution!(8);

use std::collections::HashSet;

use advent_of_code::util::list::Array2D;

fn parse_data(input: &str) -> Array2D<i32> {
    let line_size = input.lines().next().unwrap().len();
    let mut data = Array2D::new(line_size);

    for line in input.lines() {
        data.add_line(line.as_bytes().iter().map(|x| (x - b'0') as i32));
    }

    data
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let len_x = data.len_line();
    let len_y = data.len();

    let mut visible = HashSet::new();

    for y in 0..len_y {
        let mut max_tree = -1;
        for x in 0..len_x {
            if data[(x, y)] > max_tree {
                visible.insert((x, y));
                max_tree = data[(x, y)]
            }
        }
    }

    for y in 0..len_y {
        let mut max_tree = -1;
        for x in (0..len_x).rev() {
            if data[(x, y)] > max_tree {
                visible.insert((x, y));
                max_tree = data[(x, y)]
            }
        }
    }

    for x in 0..len_x {
        let mut max_tree = -1;
        for y in 0..len_y {
            if data[(x, y)] > max_tree {
                visible.insert((x, y));
                max_tree = data[(x, y)]
            }
        }
    }

    for x in 0..len_x {
        let mut max_tree = -1;
        for y in (0..len_y).rev() {
            if data[(x, y)] > max_tree {
                visible.insert((x, y));
                max_tree = data[(x, y)]
            }
        }
    }

    let result = visible.len() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let len_x = data.len_line();
    let len_y = data.len();

    let mut result = 0;

    for x in 0..len_x {
        for y in 0..len_y {
            let my_tree = data[(x, y)];

            let mut tmp_result = 1;

            let mut c = 0;
            for xx in x + 1..len_x {
                c += 1;
                if my_tree <= data[(xx, y)] {
                    break;
                }
            }
            tmp_result *= c;

            let mut c = 0;
            for xx in (0..x).rev() {
                c += 1;
                if my_tree <= data[(xx, y)] {
                    break;
                }
            }
            tmp_result *= c;

            let mut c = 0;
            for yy in y + 1..len_y {
                c += 1;
                if my_tree <= data[(x, yy)] {
                    break;
                }
            }
            tmp_result *= c;

            let mut c = 0;
            for yy in (0..y).rev() {
                c += 1;
                if my_tree <= data[(x, yy)] {
                    break;
                }
            }
            tmp_result *= c;

            result = u32::max(result, tmp_result);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
