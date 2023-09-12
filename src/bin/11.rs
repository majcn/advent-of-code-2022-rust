use regex::Regex;

#[derive(Clone, Debug)]
enum Operation {
    POW2,
    ADD(u128),
    MUL(u128),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test_devided_by: u128,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn execute_operation(&self, x: u128) -> u128 {
        match self.operation {
            Operation::POW2 => x * x,
            Operation::ADD(v) => x + v,
            Operation::MUL(v) => x * v,
        }
    }
}

fn parse_data(input: &str) -> Vec<Monkey> {
    let numbers_regex = Regex::new(r"(\d+)").unwrap();
    let function_regex = Regex::new(r"old ([*+]) (old|\d+)").unwrap();

    input
        .split("\n\n")
        .map(|description| {
            let mut description_lines = description.lines().skip(1);
            let items = numbers_regex
                .captures_iter(description_lines.next().unwrap())
                .map(|x| x[1].parse().unwrap())
                .collect();
            let operation_parameters = function_regex
                .captures(description_lines.next().unwrap())
                .unwrap();
            let test_devided_by = numbers_regex
                .captures(description_lines.next().unwrap())
                .map(|x| x[1].parse().unwrap())
                .unwrap();
            let test_true = numbers_regex
                .captures(description_lines.next().unwrap())
                .map(|x| x[1].parse().unwrap())
                .unwrap();
            let test_false = numbers_regex
                .captures(description_lines.next().unwrap())
                .map(|x| x[1].parse().unwrap())
                .unwrap();

            let operation = if operation_parameters[2].eq("old") {
                Operation::POW2
            } else if operation_parameters[1].eq("+") {
                Operation::ADD(operation_parameters[2].parse().unwrap())
            } else {
                Operation::MUL(operation_parameters[2].parse().unwrap())
            };

            Monkey {
                items,
                operation,
                test_devided_by,
                test_true,
                test_false,
            }
        })
        .collect()
}

// TODO: a se da to nekako preko iteratorja namest 0..monkeys.len()
fn part_x<const ROUNDS: usize, const WORRY_LEVEL_DIVISOR: u128>(monkeys: &[Monkey]) -> u128 {
    let mut monkeys = monkeys.to_vec(); // TODO: ne razumem zakaj ne morm dat copied, lahko pa dam clone

    let magic_number = monkeys.iter().map(|m| m.test_devided_by).product::<u128>();

    let mut inspected_items: Vec<u128> = vec![0; monkeys.len()];

    for _ in 0..ROUNDS {
        for monkey_index in 0..monkeys.len() {
            let monkey_items = std::mem::take(&mut monkeys[monkey_index].items);
            for item in monkey_items {
                inspected_items[monkey_index] += 1;
                let new_item = monkeys[monkey_index].execute_operation(item);
                let new_item = new_item / WORRY_LEVEL_DIVISOR % magic_number;
                if new_item % monkeys[monkey_index].test_devided_by == 0 {
                    let idx = monkeys[monkey_index].test_true;
                    monkeys[idx].items.push(new_item);
                } else {
                    let idx = monkeys[monkey_index].test_false;
                    monkeys[idx].items.push(new_item);
                }
            }
        }
    }

    inspected_items.sort_unstable();
    inspected_items
        .iter() // TODO: iter vs into_iter?
        .rev()
        .take(2)
        .product()
}

pub fn part_one(input: &str) -> Option<u128> {
    let data = parse_data(input);

    let result = part_x::<20, 3>(&data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    let data = parse_data(input);

    let result = part_x::<10000, 1>(&data);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
