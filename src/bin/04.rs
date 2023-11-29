advent_of_code::solution!(4);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
