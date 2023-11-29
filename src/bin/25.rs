advent_of_code::solution!(25);

fn parse_data(input: &str) -> Vec<&[u8]> {
    input.lines().map(|x| x.as_bytes()).collect()
}

fn encode(n: i64) -> String {
    let mut result = vec![];

    while decode(&result) < n {
        result.push(b'2');
    }

    for i in 0..result.len() {
        for option in [b'=', b'-', b'0', b'1', b'2'] {
            result[i] = option;
            if decode(&result) >= n {
                break;
            }
        }
    }

    String::from(std::str::from_utf8(&result).unwrap())
}

fn decode(s: &[u8]) -> i64 {
    s.iter().fold(0, |acc, v| match v {
        b'2' => 5 * acc + 2,
        b'1' => 5 * acc + 1,
        b'0' => 5 * acc,
        b'-' => 5 * acc - 1,
        b'=' => 5 * acc - 2,
        _ => unreachable!(),
    })
}

pub fn part_one(input: &str) -> Option<String> {
    let data = parse_data(input);

    let digital_sum = data.into_iter().map(decode).sum();

    let result = encode(digital_sum);

    Some(result)
}

pub fn part_two(_: &str) -> Option<String> {
    // "Thank you Eric for another wonderful year of AoC!"
    Some(String::from("⭐️⭐️"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("⭐️⭐️")));
    }
}
