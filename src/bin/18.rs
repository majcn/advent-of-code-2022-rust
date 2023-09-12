use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    const fn new((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

impl Add for &Point3 {
    type Output = Point3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn parse_data(input: &str) -> Vec<Point3> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.splitn(3, ",");
            Point3 {
                x: iter.next().unwrap().parse().unwrap(),
                y: iter.next().unwrap().parse().unwrap(),
                z: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

const fn create_side(tuples: [(i32, i32, i32); 4]) -> [Point3; 4] {
    [
        Point3::new(tuples[0]),
        Point3::new(tuples[1]),
        Point3::new(tuples[2]),
        Point3::new(tuples[3]),
    ]
}

const SIDE1: [Point3; 4] = create_side([(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]);
const SIDE2: [Point3; 4] = create_side([(0, 1, 0), (1, 1, 0), (0, 1, 1), (1, 1, 1)]);
const SIDE3: [Point3; 4] = create_side([(0, 0, 1), (1, 0, 1), (0, 1, 1), (1, 1, 1)]);
const SIDE4: [Point3; 4] = create_side([(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)]);
const SIDE5: [Point3; 4] = create_side([(1, 0, 0), (1, 1, 0), (1, 0, 1), (1, 1, 1)]);
const SIDE6: [Point3; 4] = create_side([(0, 0, 0), (0, 1, 0), (0, 0, 1), (0, 1, 1)]);
const SIDES: [[Point3; 4]; 6] = [SIDE1, SIDE2, SIDE3, SIDE4, SIDE5, SIDE6];

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut cube_sides = HashMap::new();

    for side in SIDES {
        for cube in data.iter() {
            let key = side.map(|s| cube + &s);
            match cube_sides.entry(key) {
                Entry::Occupied(mut x) => {
                    x.insert(x.get() + 1);
                }
                Entry::Vacant(x) => {
                    x.insert(1);
                }
            }
        }
    }

    let result = cube_sides.into_values().filter(|x| x == &1).count() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let cubes = data.into_iter().collect::<HashSet<Point3>>();

    let minx = cubes.iter().map(|x| x.x).min().unwrap() - 1;
    let maxx = cubes.iter().map(|x| x.x).max().unwrap() + 1;
    let miny = cubes.iter().map(|x| x.y).min().unwrap() - 1;
    let maxy = cubes.iter().map(|x| x.y).max().unwrap() + 1;
    let minz = cubes.iter().map(|x| x.z).min().unwrap() - 1;
    let maxz = cubes.iter().map(|x| x.z).max().unwrap() + 1;

    let get_neighbors_f_offsets = [
        Point3::new((1, 0, 0)),
        Point3::new((0, 1, 0)),
        Point3::new((-1, 0, 0)),
        Point3::new((0, -1, 0)),
        Point3::new((0, 0, 1)),
        Point3::new((0, 0, -1)),
    ];

    let find_group = |loc: Point3| -> HashSet<Point3> {
        let mut queue = vec![];
        let mut visited = HashSet::new();

        queue.push(loc);
        visited.insert(loc);

        while let Some(el) = queue.pop() {
            let neighbors_iter = get_neighbors_f_offsets
                .iter()
                .map(|offset| &el + offset)
                .filter(|new_loc| !cubes.contains(new_loc))
                .filter(|new_loc| (minx..=maxx).contains(&new_loc.x))
                .filter(|new_loc| (miny..=maxy).contains(&new_loc.y))
                .filter(|new_loc| (minz..=maxz).contains(&new_loc.z));

            for neighbor in neighbors_iter {
                if !visited.contains(&neighbor) {
                    queue.push(neighbor);
                    visited.insert(neighbor);
                }
            }
        }

        visited
    };

    let outter_group = find_group(Point3 {
        x: minx,
        y: miny,
        z: minz,
    });

    let mut outter_group_cube_sides = HashSet::new();
    for side in SIDES {
        for cube in outter_group.iter() {
            let key = side.map(|s| cube + &s);
            outter_group_cube_sides.insert(key);
        }
    }

    let mut result = 0;
    for side in SIDES {
        for cube in cubes.iter() {
            let key = side.map(|s| cube + &s);
            if outter_group_cube_sides.contains(&key) {
                result += 1;
            }
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
