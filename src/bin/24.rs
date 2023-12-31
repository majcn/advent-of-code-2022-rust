advent_of_code::solution!(24);

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Rem;

use advent_of_code::majcn::point::Point;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct BBox {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl BBox {
    #[inline]
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.minx && p.y <= self.maxx && p.y >= self.miny && p.y <= self.maxy
    }
}

struct Blizzard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Blizzard {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

fn parse_data(input: &str) -> (Vec<Blizzard>, BBox) {
    let bbox = BBox {
        minx: 1,
        maxx: input.lines().next().unwrap().len() as i32 - 2,
        miny: 1,
        maxy: input.lines().count() as i32 - 2,
    };

    let mut blizzards = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            match v {
                '^' => blizzards.push(Blizzard::new(x, y, Direction::Up)),
                'v' => blizzards.push(Blizzard::new(x, y, Direction::Down)),
                '<' => blizzards.push(Blizzard::new(x, y, Direction::Left)),
                '>' => blizzards.push(Blizzard::new(x, y, Direction::Right)),
                _ => {}
            }
        }
    }

    (blizzards, bbox)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GraphNode {
    location: Point,
    blizzards_index: usize,
}

fn bfs(
    start_node: GraphNode,
    end_location: Point,
    blizzard_neighbors: &mut BlizzardNeighbors,
) -> Option<(u32, GraphNode)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start_node, 0));
    visited.insert(start_node);

    while let Some((node, time)) = queue.pop_front() {
        if node.location == end_location {
            return Some((time, node));
        }

        for neighbor in blizzard_neighbors.get_neighbors(&node, &start_node.location, &end_location)
        {
            if !visited.contains(&neighbor) {
                queue.push_back((neighbor, time + 1));
                visited.insert(neighbor);
            }
        }
    }

    None
}

const NEIGHBORS_5: [Point; 5] = [
    Point { x: 0, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
];

fn get_neighbors(
    node: &GraphNode,
    blizzards_bbox: &BBox,
    blizzard_locations: &mut BlizzardLocations,
    start_location: &Point,
    end_location: &Point,
) -> Vec<GraphNode> {
    let mut result = Vec::with_capacity(5);

    let new_blizzards_index = node.blizzards_index + 1;
    let new_blizzards = blizzard_locations.get_locations(new_blizzards_index);

    for neighbor in NEIGHBORS_5 {
        let new_location = node.location + neighbor;

        if &new_location == end_location {
            return vec![GraphNode {
                location: new_location,
                blizzards_index: new_blizzards_index,
            }];
        }

        if !new_blizzards.contains(&new_location)
            && (&new_location == start_location || blizzards_bbox.contains(&new_location))
        {
            result.push(GraphNode {
                location: new_location,
                blizzards_index: new_blizzards_index,
            });
        }
    }

    result
}

fn python_like_mod<T: Add<Output = T> + Rem<Output = T> + Copy>(a: T, b: T) -> T {
    (a % b + b) % b
}

fn get_blizzard_locations(
    blizzards_bbox: &BBox,
    blizzards: &[Blizzard],
    time: usize,
) -> HashSet<Point> {
    let x_blizzard_path = blizzards_bbox.maxx - blizzards_bbox.minx + 1;
    let y_blizzard_path = blizzards_bbox.maxy - blizzards_bbox.miny + 1;

    blizzards
        .iter()
        .map(|b| match b.direction {
            Direction::Up => Point {
                x: b.x as i32,
                y: python_like_mod(b.y as i32 - 1 - time as i32, y_blizzard_path) + 1,
            },
            Direction::Down => Point {
                x: b.x as i32,
                y: python_like_mod(b.y as i32 - 1 + time as i32, y_blizzard_path) + 1,
            },
            Direction::Left => Point {
                x: python_like_mod(b.x as i32 - 1 - time as i32, x_blizzard_path) + 1,
                y: b.y as i32,
            },
            Direction::Right => Point {
                x: python_like_mod(b.x as i32 - 1 + time as i32, x_blizzard_path) + 1,
                y: b.y as i32,
            },
        })
        .collect()
}

// TODO: ni mi uspelo tega narest brez da naredim nov mut struct
struct BlizzardLocations<'a, 'b> {
    cache: HashMap<usize, HashSet<Point>>,
    blizzards: &'a [Blizzard],
    blizzards_bbox: &'b BBox,
}

impl<'a, 'b> BlizzardLocations<'a, 'b> {
    fn new(blizzards: &'a [Blizzard], blizzards_bbox: &'b BBox) -> Self {
        BlizzardLocations {
            cache: HashMap::new(),
            blizzards,
            blizzards_bbox,
        }
    }

    fn get_locations(&mut self, i: usize) -> &HashSet<Point> {
        self.cache
            .entry(i)
            .or_insert_with(|| get_blizzard_locations(self.blizzards_bbox, self.blizzards, i))
    }
}

struct BlizzardNeighbors<'a, 'b, 'c, 'd> {
    blizzards_bbox: &'a BBox,
    blizzard_locations: &'b mut BlizzardLocations<'c, 'd>,
}

impl<'a, 'b, 'c, 'd> BlizzardNeighbors<'a, 'b, 'c, 'd> {
    fn new(
        blizzards_bbox: &'a BBox,
        blizzard_locations: &'b mut BlizzardLocations<'c, 'd>,
    ) -> Self {
        BlizzardNeighbors {
            blizzards_bbox,
            blizzard_locations,
        }
    }

    fn get_neighbors(
        &mut self,
        node: &GraphNode,
        start_location: &Point,
        end_location: &Point,
    ) -> Vec<GraphNode> {
        get_neighbors(
            node,
            self.blizzards_bbox,
            self.blizzard_locations,
            start_location,
            end_location,
        )
    }
}

fn part_x(blizzards: &[Blizzard], blizzards_bbox: &BBox, path: &[Point]) -> u32 {
    let mut result = 0;

    let current_blizzard_index = 0;
    let mut node = GraphNode {
        location: path[0],
        blizzards_index: current_blizzard_index,
    };

    for p in path.iter().skip(1) {
        // TODO: zakaj ne morm funkcije vrzt v svojo spremenljivko? compiler tezi...
        let mut blizzard_locations = BlizzardLocations::new(blizzards, blizzards_bbox);
        let mut blizzard_neighbors =
            BlizzardNeighbors::new(blizzards_bbox, &mut blizzard_locations);

        let bfs_result = bfs(node, *p, &mut blizzard_neighbors);

        if let Some((time, goal_node)) = bfs_result {
            node = goal_node;
            result += time;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (blizzards, blizzards_bbox) = parse_data(input);

    let path = [
        Point { x: 1, y: 0 },
        Point {
            x: blizzards_bbox.maxx,
            y: blizzards_bbox.maxy + 1,
        },
    ];

    let result = part_x(&blizzards, &blizzards_bbox, &path);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (blizzards, blizzards_bbox) = parse_data(input);

    let path = [
        Point { x: 1, y: 0 },
        Point {
            x: blizzards_bbox.maxx,
            y: blizzards_bbox.maxy + 1,
        },
        Point { x: 1, y: 0 },
        Point {
            x: blizzards_bbox.maxx,
            y: blizzards_bbox.maxy + 1,
        },
    ];

    let result = part_x(&blizzards, &blizzards_bbox, &path);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
