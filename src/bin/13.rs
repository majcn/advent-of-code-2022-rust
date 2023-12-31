advent_of_code::solution!(13);

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Node {
    Array(Vec<Node>),
    Value(u32),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

fn parse_package(package: &str) -> Node {
    let package = package.as_bytes();

    let mut stack = vec![];
    let mut i = 0;

    // TODO: ce ma kdo idejo kako se da tole brez i-ja resit. mogoce peakable?
    while i < package.len() - 1 {
        match package[i] {
            b'[' => stack.push(vec![]),
            b']' => {
                let value = stack.pop().unwrap();
                stack.last_mut().unwrap().push(Node::Array(value));
            }
            b'0'..=b'9' => {
                let mut number = 0;
                for x in package[i..].iter() {
                    if x.is_ascii_digit() {
                        number = (number * 10) + (x - b'0') as u32;
                        i += 1;
                    } else {
                        break;
                    }
                }
                stack.last_mut().unwrap().push(Node::Value(number));
                i -= 1;
            }
            _ => {}
        }

        i += 1;
    }

    Node::Array(stack.pop().unwrap())
}

fn parse_data(input: &str) -> Vec<(Node, Node)> {
    let result = input
        .split("\n\n")
        .map(|x| {
            let (left, right) = x.split_once('\n').unwrap();
            (parse_package(left), parse_package(right))
        })
        .collect();

    result
}

fn compare(left: &Node, right: &Node) -> Ordering {
    let (left_iterator, left_size): (Box<dyn Iterator<Item = &Node>>, usize) = match left {
        Node::Array(v) => (Box::new(v.iter()), v.len()),
        Node::Value(_) => (Box::new(std::iter::once(left)), 1),
    };

    let (right_iterator, right_size): (Box<dyn Iterator<Item = &Node>>, usize) = match right {
        Node::Array(v) => (Box::new(v.iter()), v.len()),
        Node::Value(_) => (Box::new(std::iter::once(right)), 1),
    };

    for (left_el, right_el) in left_iterator.zip(right_iterator) {
        let result = match (left_el, right_el) {
            (Node::Value(vl), Node::Value(vr)) => Ord::cmp(vl, vr),
            _ => compare(left_el, right_el),
        };

        if result != Ordering::Equal {
            return result;
        }
    }

    left_size.cmp(&right_size)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .enumerate()
        .filter(|(_, x)| x.0.lt(&x.1))
        .map(|x| (x.0 + 1) as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    enum NodeWithMark {
        Package(Node),
        Divider(Node),
    }

    let data = parse_data(input);

    let divider1 = Node::Array(vec![Node::Array(vec![Node::Value(2)])]);
    let divider2 = Node::Array(vec![Node::Array(vec![Node::Value(6)])]);

    let mut all_packages = Vec::with_capacity(data.len() * 2 + 2);
    for (left, right) in data {
        all_packages.push(NodeWithMark::Package(left));
        all_packages.push(NodeWithMark::Package(right));
    }
    all_packages.push(NodeWithMark::Divider(divider1));
    all_packages.push(NodeWithMark::Divider(divider2));

    all_packages.sort_unstable_by(|left, right| {
        let left = match left {
            NodeWithMark::Package(v) => v,
            NodeWithMark::Divider(v) => v,
        };

        let right = match right {
            NodeWithMark::Package(v) => v,
            NodeWithMark::Divider(v) => v,
        };

        Ord::cmp(left, right)
    });

    let result = all_packages
        .iter()
        .enumerate()
        .filter(|x| matches!(x.1, NodeWithMark::Divider(_)))
        .map(|x| (x.0 + 1) as u32)
        .product();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }
}
