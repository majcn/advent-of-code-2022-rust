advent_of_code::solution!(6);

use std::collections::HashSet;

fn parse_data(input: &str) -> &[u8] {
    input.as_bytes()
}

fn part_x<const N: usize>(data: &[u8]) -> u32 {
    data.windows(N)
        .position(|w| w.iter().collect::<HashSet<&u8>>().len() == N) // TODO: zakaj to dela? .copied()?
        .map(|x| x + N)
        .unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<4>(data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<14>(data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
