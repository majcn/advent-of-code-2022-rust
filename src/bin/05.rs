advent_of_code::solution!(5);

use regex::Regex;

use advent_of_code::majcn::parse::ParseRegex;

struct Command {
    n: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;
type State = Vec<Stack>;

fn parse_data(input: &str) -> (State, Vec<Command>) {
    let (state_lines, command_lines) = input.split_once("\n\n").unwrap();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let commands = command_lines
        .lines()
        .map(|x| re.parse_usize(x))
        .map(|[n, from, to]| Command {
            n,
            from: from - 1,
            to: to - 1,
        })
        .collect();

    let state_lines: Vec<&str> = state_lines.lines().rev().skip(1).collect();

    let mut state: Vec<Vec<char>> = vec![vec![]; (state_lines[0].len() + 1) / 4];
    for row in state_lines {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c.is_whitespace() {
                continue;
            }

            state[i].push(c);
        }
    }

    (state, commands)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut state, commands) = parse_data(input);

    for command in commands {
        let mut stack_from = std::mem::take(&mut state[command.from]);
        let crates = stack_from.drain(stack_from.len() - command.n..);

        state[command.to].extend(crates.rev());
        state[command.from] = stack_from;
    }

    let result = state.iter().map(|x| x.last().unwrap()).collect();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut state, commands) = parse_data(input);

    for command in commands {
        let mut stack_from = std::mem::take(&mut state[command.from]);
        let crates = stack_from.drain(stack_from.len() - command.n..);

        state[command.to].extend(crates);
        state[command.from] = stack_from;
    }

    let result = state.iter().map(|x| x.last().unwrap()).collect();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("MCD")));
    }
}
