advent_of_code::solution!(12);

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::majcn::point::Point;
use advent_of_code::majcn::point::DOWN;
use advent_of_code::majcn::point::LEFT;
use advent_of_code::majcn::point::RIGHT;
use advent_of_code::majcn::point::UP;

struct Node {
    mark: u8,
    neighbors: Vec<Point>,
}

impl Node {
    fn new(mark: u8) -> Self {
        Node {
            mark,
            neighbors: Vec::with_capacity(4),
        }
    }
}

type Arena = HashMap<Point, Node>;

fn parse_data(input: &str) -> (Point, Point, Arena) {
    let mut arena = HashMap::with_capacity(input.len());
    let mut start_node = Point::new(0, 0);
    let mut end_node = Point::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, v) in line.as_bytes().iter().enumerate() {
            let x = x as i32; // TODO: se da to lepse napisat?
            let node = match v {
                b'S' => {
                    start_node = Point::new(x, y);
                    Node::new(b'a')
                }
                b'E' => {
                    end_node = Point::new(x, y);
                    Node::new(b'z')
                }
                _ => Node::new(*v),
            };
            arena.insert(Point::new(x, y), node);
        }
    }

    // TODO: again... a je potrebno tale copy + collect?
    // jamra zaradi mutacije v arena
    let all_locations = arena.keys().copied().collect::<Vec<_>>();

    for p in all_locations {
        let mut p_node = arena.remove(&p).unwrap();
        for direction in [LEFT, RIGHT, UP, DOWN] {
            let new_p = p + direction;
            if arena.contains_key(&new_p) {
                let new_p_node = arena.get(&new_p).unwrap();
                if new_p_node.mark < p_node.mark || new_p_node.mark - p_node.mark < 2 {
                    p_node.neighbors.push(new_p);
                }
            }
        }
        arena.insert(p, p_node);
    }

    (start_node, end_node, arena)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct AStarNode<'a> {
    cost: u32,
    node_index: &'a Point,
}

impl<'a> PartialOrd for AStarNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for AStarNode<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// TODO: tole je dobra funkcija da dojames kaj in kako z & in *
fn a_star_search_algorithm(arena: &Arena, start_node: &Point, end_node: &Point) -> Option<u32> {
    let h = |n: &Point| ((end_node.x - n.x).pow(2) + (end_node.y - n.y).pow(2)) as u32;
    let d = |_c: &Point, _n: &Point| 1;

    let mut open_set = HashSet::new();
    open_set.insert(start_node); // TODO: probej ce dela brez *; zakaj dela?

    let mut open_set_queue = BinaryHeap::new();
    open_set_queue.push(AStarNode {
        cost: 0,
        node_index: start_node,
    });

    let mut g_score = HashMap::new(); // TODO: probej ce dela brez *
    g_score.insert(start_node, 0);

    // TODO: preveri kje se da se to ponucat
    while let Some(AStarNode { cost, node_index }) = open_set_queue.pop() {
        let current = node_index;

        if current == end_node {
            // TODO: kaj dejansko tukaj naredi? prej sm imel value, zdaj imam reference. isto dela
            return Some(cost);
        }

        open_set.remove(current);
        for neighbor_index in arena[current].neighbors.iter() {
            let tentative_g_score = g_score.get(current).unwrap() + d(current, neighbor_index);
            let g_score_value = g_score.get(neighbor_index).copied().unwrap_or(u32::MAX); // TODO: je treba kopirat?
            if tentative_g_score >= g_score_value {
                continue;
            }

            g_score.insert(neighbor_index, tentative_g_score);
            if !open_set.contains(neighbor_index) {
                open_set.insert(neighbor_index);
                open_set_queue.push(AStarNode {
                    cost: tentative_g_score + h(neighbor_index),
                    node_index: neighbor_index,
                })
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_node, end_node, arena) = parse_data(input);

    let result = a_star_search_algorithm(&arena, &start_node, &end_node).unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, end_node, arena) = parse_data(input);

    // TODO: OK ne razumem vec... zakaj pa tukaj lahko &arena posljem naprej?
    let result = arena
        .iter()
        .filter(|x| x.1.mark == b'a')
        .filter_map(|x| a_star_search_algorithm(&arena, x.0, &end_node))
        .min();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
