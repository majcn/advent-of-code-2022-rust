use std::collections::HashMap;

fn parse_data(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|x| x.as_bytes())
        .map(|x| (x[0], x[2]))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let score = HashMap::from([
        ((b'A', b'X'), 3 + 1),
        ((b'A', b'Y'), 6 + 2),
        ((b'A', b'Z'), 0 + 3),
        ((b'B', b'X'), 0 + 1),
        ((b'B', b'Y'), 3 + 2),
        ((b'B', b'Z'), 6 + 3),
        ((b'C', b'X'), 6 + 1),
        ((b'C', b'Y'), 0 + 2),
        ((b'C', b'Z'), 3 + 3),
    ]);

    let result = data.iter().map(|x| score[x]).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let score = HashMap::from([
        ((b'A', b'X'), 0 + 3),
        ((b'A', b'Y'), 3 + 1),
        ((b'A', b'Z'), 6 + 2),
        ((b'B', b'X'), 0 + 1),
        ((b'B', b'Y'), 3 + 2),
        ((b'B', b'Z'), 6 + 3),
        ((b'C', b'X'), 0 + 2),
        ((b'C', b'Y'), 3 + 3),
        ((b'C', b'Z'), 6 + 1),
    ]);

    let result = data.iter().map(|x| score[x]).sum();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
