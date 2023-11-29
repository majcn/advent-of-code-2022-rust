advent_of_code::solution!(3);

use std::collections::BTreeSet;

fn parse_data(input: &str) -> Vec<&[u8]> {
    input.lines().map(|x| x.as_bytes()).collect()
}

fn priority(s: u8) -> u32 {
    if s.is_ascii_lowercase() {
        (s as u32) - 96
    } else {
        (s as u32) - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|x| {
            let p1 = x[..x.len() / 2].iter().collect::<BTreeSet<&u8>>();
            let p2 = x[x.len() / 2..].iter().collect::<BTreeSet<&u8>>();

            let mut intersection_iter = p1.intersection(&p2);
            let element = intersection_iter.next().unwrap();
            priority(**element)
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .chunks_exact(3)
        .map(|c| {
            let p1 = c[0].iter().collect::<BTreeSet<&u8>>();
            let p2 = c[1].iter().collect::<BTreeSet<&u8>>();
            let p3 = c[2].iter().collect::<BTreeSet<&u8>>();

            let intersection = p2.intersection(&p3).copied().collect(); // TODO: zakaj mora biti tukaj copied; bi se dalo brez tega?
            let mut intersection_iter = p1.intersection(&intersection);
            let element = intersection_iter.next().unwrap();
            priority(**element)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
