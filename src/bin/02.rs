advent_of_code::solution!(2);

fn parse_data(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|x| x.as_bytes())
        .map(|x| (x[0].into(), x[2].into()))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    // TODO: const fn VS fn?
    fn score((x, y): (char, char)) -> u32 {
        match (x, y) {
            ('A', 'X') => 4,
            ('A', 'Y') => 8,
            ('A', 'Z') => 3,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 7,
            ('C', 'Y') => 2,
            ('C', 'Z') => 6,
            _ => unreachable!(),
        }
    }

    let result = data.into_iter().map(score).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    fn score((x, y): (char, char)) -> u32 {
        match (x, y) {
            ('A', 'X') => 3,
            ('A', 'Y') => 4,
            ('A', 'Z') => 8,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 2,
            ('C', 'Y') => 6,
            ('C', 'Z') => 7,
            _ => unreachable!(),
        }
    }

    let result = data.into_iter().map(score).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
