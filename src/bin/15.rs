advent_of_code::solution!(15);

use regex::Regex;
use std::collections::BTreeSet;

use advent_of_code::majcn::parse::ParseRegex;
use advent_of_code::majcn::point::Point;

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn new(min: i32, max: i32) -> Self {
        Range { min, max }
    }
}

struct Sensor {
    point: Point,
    closest_beacon: Point,
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }
}

impl Line {
    fn intersection(&self, other: &Self) -> Option<Point> {
        #[inline]
        fn det(a: Point, b: Point) -> i32 {
            (a.x as i64 * b.y as i64 - a.y as i64 * b.x as i64) as i32
        }

        let x_diff = Point::new(self.p1.x - self.p2.x, other.p1.x - other.p2.x);
        let y_diff = Point::new(self.p1.y - self.p2.y, other.p1.y - other.p2.y);

        let div = det(x_diff, y_diff);
        if div == 0 {
            return None;
        }

        let d = Point::new(det(self.p1, self.p2), det(other.p1, other.p2));

        let x = det(d, x_diff) / div;
        let y = det(d, y_diff) / div;

        Some(Point::new(x, y))
    }
}

fn parse_data(input: &str) -> Vec<Sensor> {
    let re_str = r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$";
    let re = Regex::new(re_str).unwrap();

    // TODO: a je kaksna sansa da se ne unwrapa za vsako stvar? sej vem da gotovo ne :) ampak sam tok ce se slucajno da
    input
        .lines()
        .map(|x| re.parse_i32(x))
        .map(|[x, y, bx, by]| Sensor {
            point: Point::new(x, y),
            closest_beacon: Point::new(bx, by),
        })
        .collect()
}

fn part_x(data: &[Sensor], y: i32) -> Vec<Range> {
    let mut ranges = Vec::with_capacity(data.len());
    for sensor in data {
        let max_distance = i32::abs(sensor.point.x - sensor.closest_beacon.x)
            + i32::abs(sensor.point.y - sensor.closest_beacon.y);
        let y_distance = i32::abs(sensor.point.y - y);
        if max_distance > y_distance {
            let diff = max_distance - y_distance;
            ranges.push(Range::new(sensor.point.x - diff, sensor.point.x + diff));
        }
    }

    ranges.sort_unstable_by_key(|x| x.min);

    loop {
        let mut new_ranges = Vec::with_capacity(ranges.len());
        new_ranges.extend(ranges.drain(..1));

        let mut no_changes = true;
        for r2 in ranges.into_iter() {
            let r1 = new_ranges.pop().unwrap();

            if r2.min < r1.min && r1.max < r2.max {
                new_ranges.push(r2);
                no_changes = false;
            } else if r1.min <= r2.min && r2.max <= r1.max {
                new_ranges.push(r1);
                no_changes = false;
            } else if r1.min == r2.min {
                new_ranges.push(Range::new(r1.min, i32::max(r1.max, r2.max)));
                no_changes = false;
            } else if r1.max == r2.max {
                new_ranges.push(Range::new(i32::min(r1.min, r2.min), r1.max));
                no_changes = false;
            } else if r2.min <= r1.max && r1.min <= r2.min {
                new_ranges.push(Range::new(r1.min, r2.max));
                no_changes = false;
            } else if r1.min < r2.max && r2.min < r1.min {
                new_ranges.push(Range::new(r2.min, r1.max));
                no_changes = false;
            } else if r1.max < r2.min {
                new_ranges.push(r1);
                new_ranges.push(r2);
            }
        }

        if no_changes {
            return new_ranges;
        }

        ranges = new_ranges;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    const Y: i32 = 2000000;

    let result = part_x(&data, Y)
        .into_iter()
        .map(|r| r.max - r.min + 1)
        .sum::<i32>() as u64;

    let ignore_beacons_count = data
        .into_iter()
        .map(|s| s.closest_beacon)
        .filter(|b| b.y == Y)
        .collect::<BTreeSet<_>>()
        .len() as u64;

    let result = result - ignore_beacons_count;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    const MIN_Y: i32 = 0;
    const MAX_Y: i32 = 4000000;

    let mut lines = Vec::with_capacity(data.len() * 4);
    for sensor in data.iter() {
        let max_distance = Point::manhattan(sensor.point, sensor.closest_beacon);
        let left_point = Point::new(sensor.point.x - max_distance - 1, sensor.point.y);
        let right_point = Point::new(sensor.point.x - max_distance + 1, sensor.point.y);

        lines.push(Line::new(left_point, left_point + Point::new(1, -1)));
        lines.push(Line::new(left_point, left_point + Point::new(1, 1)));
        lines.push(Line::new(right_point, right_point + Point::new(-1, -1)));
        lines.push(Line::new(right_point, right_point + Point::new(-1, 1)));
    }

    let mut interesting_points = BTreeSet::new();
    for line1 in lines.iter() {
        for line2 in lines.iter() {
            if let Some(p) = line1.intersection(line2) {
                if let MIN_Y..=MAX_Y = p.y {
                    interesting_points.insert(p.y);
                }
            }
        }
    }

    let result = interesting_points
        .into_iter()
        .map(|y| (y, part_x(&data, y)))
        .find(|(_, r)| r.len() == 2)
        .map(|(y, r)| (r[0].max + 1) as u64 * MAX_Y as u64 + y as u64)
        .unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3075235));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2746461376372));
    }
}
