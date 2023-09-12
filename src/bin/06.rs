use std::collections::HashSet;

fn parse_data(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part_x<const N: usize>(data: &[char]) -> u32 {
    data.windows(N)
        .position(|w| w.iter().collect::<HashSet<&char>>().len() == N) // TODO: zakaj to dela? .copied()?
        .map(|x| x + N)
        .unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<4>(&data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<14>(&data);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
