advent_of_code::solution!(21);

use regex::Regex;
use std::collections::HashMap;

type NodeId = [u8; 4];

enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

enum Node {
    ValueNode(u64),
    EquationNode(NodeId, NodeId, Operator),
}

fn parse_data(input: &str) -> HashMap<NodeId, Node> {
    let re_str_equation: &str = r"^(\w{4}): (\w{4}) ([+\-*/]) (\w{4})$";
    let re_equation = Regex::new(re_str_equation).unwrap();

    let re_str_value: &str = r"(^\w{4}): (\d+)$";
    let re_value = Regex::new(re_str_value).unwrap();

    input
        .lines()
        .map(|x| {
            if x.len() == 17 {
                let captures = re_equation.captures(x).unwrap();

                let left = captures[2].as_bytes().try_into().unwrap();
                let right = captures[4].as_bytes().try_into().unwrap();
                let operator = match &captures[3] {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    _ => unreachable!(),
                };

                let key = captures[1].as_bytes().try_into().unwrap();
                let value = Node::EquationNode(left, right, operator);
                (key, value)
            } else {
                let captures = re_value.captures(x).unwrap();

                let key = captures[1].as_bytes().try_into().unwrap();
                let value = Node::ValueNode(captures[2].parse().unwrap());
                (key, value)
            }
        })
        .collect()
}

fn solve_value(value: &u64) -> f64 {
    *value as f64
}

fn solve_equation(
    data: &HashMap<NodeId, Node>,
    left: &NodeId,
    right: &NodeId,
    op: &Operator,
) -> f64 {
    let left_solved = solve_node(data, left);
    let right_solved = solve_node(data, right);

    match op {
        Operator::Mul => left_solved * right_solved,
        Operator::Div => left_solved / right_solved,
        Operator::Add => left_solved + right_solved,
        Operator::Sub => left_solved - right_solved,
    }
}

fn solve_node(data: &HashMap<NodeId, Node>, node_id: &NodeId) -> f64 {
    let node = &data[node_id];

    match node {
        Node::ValueNode(v) => solve_value(v),
        Node::EquationNode(left, right, op) => solve_equation(data, left, right, op),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let root_id: NodeId = "root".as_bytes().try_into().unwrap();

    let result = solve_node(&data, &root_id) as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data = parse_data(input);

    let root_id: NodeId = "root".as_bytes().try_into().unwrap();
    let humn_id: NodeId = "humn".as_bytes().try_into().unwrap();

    let (left_id, right_id) = match data[&root_id] {
        Node::EquationNode(left, right, _) => (left, right),
        Node::ValueNode(_) => return None,
    };
    let right = solve_node(&data, &right_id);

    let mut a = 0;
    let mut b = u64::MAX;
    loop {
        let c = (a + b) / 2;

        data.insert(humn_id, Node::ValueNode(c));
        let left_c = solve_node(&data, &left_id) - right;

        if left_c == 0_f64 {
            return Some(c);
        }

        data.insert(humn_id, Node::ValueNode(a));
        let left_a = solve_node(&data, &left_id) - right;

        if left_a.signum() == left_c.signum() {
            a = c;
        } else {
            b = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(152));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(301));
    }
}
