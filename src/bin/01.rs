fn parse_data(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn part_x(data: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result: Vec<u32> = data.iter().map(|x| x.iter().sum()).collect();
    result.sort_unstable();
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data).last().copied();

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data).iter().rev().take(3).sum(); // TODO: iter vs into_iter?

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
