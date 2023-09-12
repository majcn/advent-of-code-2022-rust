use std::collections::BTreeSet;

fn parse_data(input: &str) -> Vec<&[u8]> {
    input.lines().map(|x| x.as_bytes()).collect()
}

fn priority(s: u8) -> u32 {
    if (b'a'..=b'z').contains(&s) {
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
        .chunks(3)
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

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
