use regex::Regex;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use advent_of_code::util::parse::ParseRegex;

fn parse_data(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    input
        .lines()
        .map(|x| re.parse_u32(x))
        .map(|[r1_min, r1_max, r2_min, r2_max]| (r1_min..=r1_max, r2_min..=r2_max))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|(r1, r2)| (r1.collect::<BTreeSet<u32>>(), r2.collect::<BTreeSet<u32>>()))
        .filter(|(s1, s2)| {
            if s1.len() > s2.len() {
                s2.is_subset(s1)
            } else {
                s1.is_subset(s2)
            }
        })
        .count() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|(r1, r2)| (r1.collect::<BTreeSet<u32>>(), r2.collect::<BTreeSet<u32>>()))
        .filter(|(s1, s2)| !s1.is_disjoint(s2))
        .count() as u32;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
