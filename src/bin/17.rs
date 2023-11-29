advent_of_code::solution!(17);

use std::collections::hash_map::Entry;
use std::collections::hash_map::HashMap;

use advent_of_code::util::point::Point;

mod tetris {
    use advent_of_code::util::point::Point;
    use advent_of_code::util::point::DOWN;
    use advent_of_code::util::point::LEFT;
    use advent_of_code::util::point::RIGHT;

    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::Hasher;

    type Rock = [Point; 5];

    pub struct Game {
        solid_rocks: HashMap<i32, u8>,
        moving_rock: Rock,
        moving_rock_location: Point,
        max_grid_x: i32,
        max_grid_y: i32,
    }

    impl Game {
        pub fn new(init_rock: Rock) -> Self {
            Game {
                solid_rocks: HashMap::new(),
                moving_rock: init_rock,
                moving_rock_location: Point { x: 2, y: 3 },
                max_grid_x: 6,
                max_grid_y: -1,
            }
        }

        pub fn place_rock(&mut self, rock: Rock) {
            self.moving_rock = rock;
            self.moving_rock_location = Point {
                x: 2,
                y: self.max_grid_y + 4,
            }
        }

        pub fn transform_to_solid(&mut self) {
            for part in self.moving_rock.iter() {
                let xx = self.moving_rock_location.x + part.x;
                let yy = self.moving_rock_location.y + part.y;

                let solid_rocks_yy = self.solid_rocks.entry(yy).or_insert(0);
                *solid_rocks_yy |= 2 << xx;

                self.max_grid_y = i32::max(self.max_grid_y, yy);
            }
        }

        pub fn step(&mut self, direction: u8) -> bool {
            match direction {
                b'>' => {
                    let to = self.moving_rock_location + RIGHT;
                    if self.can_move(&to) {
                        self.moving_rock_location.x += 1;
                    }
                }
                b'<' => {
                    let to = self.moving_rock_location + LEFT;
                    if self.can_move(&to) {
                        self.moving_rock_location.x -= 1;
                    }
                }
                _ => {}
            }

            let to = self.moving_rock_location + DOWN;
            if self.can_move(&to) {
                self.moving_rock_location.y -= 1;
                return true;
            }

            false
        }

        fn can_move(&self, to: &Point) -> bool {
            if to.y == -1 {
                return false;
            }

            for part in self.moving_rock.iter() {
                let xx = part.x + to.x;
                if xx == -1 || xx == self.max_grid_x + 1 {
                    return false;
                }

                let yy = part.y + to.y;
                if let Some(solid_rocks_yy) = self.solid_rocks.get(&yy) {
                    if solid_rocks_yy & 2 << xx > 0 {
                        return false;
                    }
                }
            }

            true
        }

        pub fn score(&self) -> u32 {
            (self.max_grid_y + 1) as u32
        }

        pub fn solid_rocks_hashable<const SIZE: u32>(&self) -> u64 {
            let min_y = self.max_grid_y - SIZE as i32;
            if min_y <= 0 {
                return 0;
            }

            let mut hasher = DefaultHasher::new();

            for y in min_y..=self.max_grid_y {
                hasher.write_u8(self.solid_rocks[&y]);
                hasher.write_i32(y - min_y);
            }

            hasher.finish()
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    solid_rocks: u64,
    direction_index: usize,
    current_rock_index: usize,
}

struct CacheValue {
    number_of_solid_rocks: u32,
    score: u32,
}

type Cache = HashMap<CacheKey, CacheValue>;

fn parse_data(input: &str) -> &[u8] {
    input.as_bytes()
}

const fn as_points(tuples: [(i32, i32); 5]) -> [Point; 5] {
    [
        Point::new(tuples[0].0, tuples[0].1),
        Point::new(tuples[1].0, tuples[1].1),
        Point::new(tuples[2].0, tuples[2].1),
        Point::new(tuples[3].0, tuples[3].1),
        Point::new(tuples[4].0, tuples[4].1),
    ]
}

fn part_x<const DURATION: u64, const USE_CACHE: bool>(data: &[u8]) -> u64 {
    const ROCKS: [[Point; 5]; 5] = [
        as_points([(0, 0), (0, 0), (1, 0), (2, 0), (3, 0)]),
        as_points([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        as_points([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        as_points([(0, 0), (0, 0), (0, 1), (0, 2), (0, 3)]),
        as_points([(0, 0), (0, 0), (1, 0), (0, 1), (1, 1)]),
    ];

    let mut game_duration = DURATION;
    let mut cache_disabled = !USE_CACHE;

    let mut game = tetris::Game::new(ROCKS[0]);

    let mut number_of_solid_rocks = 0;
    let mut current_rock_index = 1;

    let mut cache = Cache::new();
    let mut additional_score = 0;

    let mut i = 0;
    while number_of_solid_rocks as u64 != game_duration {
        let direction_index = i % data.len();
        let did_move = game.step(data[direction_index]);
        if !did_move {
            game.transform_to_solid();
            game.place_rock(ROCKS[current_rock_index]);
            current_rock_index = (current_rock_index + 1) % ROCKS.len();
            number_of_solid_rocks += 1;

            if cache_disabled {
                i += 1;
                continue;
            }

            let new_cache_key = CacheKey {
                solid_rocks: game.solid_rocks_hashable::<100>(),
                direction_index,
                current_rock_index,
            };

            let new_cache_value = CacheValue {
                number_of_solid_rocks,
                score: game.score(),
            };

            match cache.entry(new_cache_key) {
                Entry::Occupied(prev_cache_value) => {
                    let prev_cache_value = prev_cache_value.get();
                    let diff_number_of_solid_rocks = new_cache_value.number_of_solid_rocks
                        - prev_cache_value.number_of_solid_rocks;
                    let diff_score = new_cache_value.score - prev_cache_value.score;

                    let multiplier = (game_duration - number_of_solid_rocks as u64)
                        / diff_number_of_solid_rocks as u64;
                    game_duration -= diff_number_of_solid_rocks as u64 * multiplier;
                    additional_score = diff_score as u64 * multiplier;
                    cache_disabled = true;
                }
                Entry::Vacant(o) => {
                    o.insert(new_cache_value);
                }
            }
        }

        i += 1;
    }

    game.score() as u64 + additional_score
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<2022, false>(data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<1000000000000, true>(data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3068));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514285714288));
    }
}
