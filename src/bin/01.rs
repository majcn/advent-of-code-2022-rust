advent_of_code::solution!(1);

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

    let result = part_x(data).into_iter().next_back().unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data).into_iter().rev().take(3).sum(); // TODO: iter vs into_iter?

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
