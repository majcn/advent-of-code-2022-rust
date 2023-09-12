use regex::Regex;
use std::str;

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
    let commands: Vec<Command> = command_lines
        .lines()
        .map(|x| {
            let captures = re.captures(x).unwrap();
            Command {
                n: captures[1].parse().unwrap(),
                from: captures[2].parse::<usize>().unwrap() - 1,
                to: captures[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    let state_lines: Vec<&str> = state_lines.lines().rev().skip(1).collect();

    let mut state: Vec<Vec<char>> = vec![Vec::new(); (state_lines[0].len() + 1) / 4];
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
    let (state_original, commands) = parse_data(input);
    // let mut state = Vec::with_capacity(state_original.len()); // TODO: a se do kaj kej lepse resit? obcutek imam da ce bi dal samo clone da ne bi kloniral inner vectorja
    // for state_original_el in state_original {
    //     state.push(state_original_el.clone()); // TODO: saj to naredi kopijo celotnega lista?
    // }
    let mut state = state_original.iter().cloned().collect::<Vec<_>>(); // TODO: preveri ce to sploh dela na tak nacin

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
    let (state_original, commands) = parse_data(input);
    let mut state = state_original.iter().cloned().collect::<Vec<_>>(); // TODO: preveri ce to sploh dela na tak nacin

    for command in commands {
        let mut stack_from = std::mem::take(&mut state[command.from]);
        let crates = stack_from.drain(stack_from.len() - command.n..);

        state[command.to].extend(crates);
        state[command.from] = stack_from;
    }

    let result = state.iter().map(|x| x.last().unwrap()).collect();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
