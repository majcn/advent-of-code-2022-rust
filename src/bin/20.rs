mod list {
    pub struct CircualList {
        raw_data: Vec<CircualListNode>,
        available_slots: Vec<usize>,
    }

    struct CircualListNode {
        active: bool,
        pub idx: usize,
        pub value: i64,
        pub prev_index: usize,
        pub next_index: usize,
    }

    impl CircualList {
        pub fn new(data: &[i64]) -> Self {
            let nodes = data
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if i == 0 {
                        CircualListNode {
                            active: true,
                            idx: i,
                            value: *v,
                            prev_index: data.len() - 1,
                            next_index: i + 1,
                        }
                    } else if i == data.len() - 1 {
                        CircualListNode {
                            active: true,
                            idx: i,
                            value: *v,
                            prev_index: i - 1,
                            next_index: 0,
                        }
                    } else {
                        CircualListNode {
                            active: true,
                            idx: i,
                            value: *v,
                            prev_index: i - 1,
                            next_index: i + 1,
                        }
                    }
                })
                .collect::<Vec<_>>();

            CircualList {
                raw_data: nodes,
                available_slots: vec![],
            }
        }

        pub fn get_node_value(&self, idx: usize) -> i64 {
            self.raw_data[idx].value
        }

        pub fn get_node_next(&self, idx: usize) -> usize {
            self.raw_data[idx].next_index
        }

        pub fn get_node_prev(&self, idx: usize) -> usize {
            self.raw_data[idx].prev_index
        }

        pub fn insert_after(&mut self, after_index: usize, value: i64) -> usize {
            let after = &self.raw_data[after_index];

            let (idx, can_replace) = match self.available_slots.pop() {
                Some(slot) => (slot, true),
                _ => (self.raw_data.len(), false),
            };

            let node = CircualListNode {
                active: true,
                idx,
                value,
                prev_index: after.idx,
                next_index: after.next_index,
            };

            self.raw_data[node.prev_index].next_index = node.idx;
            self.raw_data[node.next_index].prev_index = node.idx;

            if can_replace {
                self.raw_data[idx] = node;
            } else {
                self.raw_data.push(node);
            }

            idx
        }

        pub fn remove_node(&mut self, node_index: usize) {
            let CircualListNode {
                prev_index,
                next_index,
                ..
            } = self.raw_data[node_index];

            self.raw_data[node_index].active = false;
            self.raw_data[prev_index].next_index = next_index;
            self.raw_data[next_index].prev_index = prev_index;

            self.available_slots.push(node_index);
        }

        pub fn iter(&self) -> CircualListIter {
            CircualListIter {
                data: self,
                node_index: self.raw_data.iter().find(|x| x.active).unwrap().idx,
            }
        }
    }

    pub struct CircualListIter<'a> {
        data: &'a CircualList,
        node_index: usize,
    }

    impl<'a> Iterator for CircualListIter<'a> {
        type Item = i64;

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.data.raw_data.get(self.node_index);

            self.node_index = self.data.raw_data[self.node_index].next_index;

            result.map(|x| x.value)
        }
    }
}

fn parse_data(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part_x<const N: usize>(data: &[i64]) -> u64 {
    let mut my_list = list::CircualList::new(data);
    let mut my_ids = (0..data.len()).into_iter().collect::<Vec<_>>();

    for _ in 0..N {
        for i in 0..my_ids.len() {
            let node_idx = my_ids[i];
            let node_value = my_list.get_node_value(node_idx);

            if node_value > 0 {
                my_list.remove_node(node_idx);

                let mut my_next_node_index = my_list.get_node_next(node_idx);
                let repeat = (node_value % (my_ids.len() as i64 - 1)) - 1;
                for _ in 0..repeat {
                    my_next_node_index = my_list.get_node_next(my_next_node_index);
                }

                my_ids[i] = my_list.insert_after(my_next_node_index, node_value);
            } else if node_value < 0 {
                my_list.remove_node(node_idx);

                let mut target_index = my_list.get_node_prev(node_idx);
                let repeat = node_value.abs() % (my_ids.len() as i64 - 1);
                for _ in 0..repeat {
                    target_index = my_list.get_node_prev(target_index);
                }

                my_ids[i] = my_list.insert_after(target_index, node_value);
            }
        }
    }

    let result = my_list
        .iter()
        .skip_while(|x| x != &0)
        .take(3001)
        .step_by(1000)
        .sum::<i64>();

    result as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<1>(&data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);
    let data = data.into_iter().map(|x| x * 811589153).collect::<Vec<_>>();

    let result = part_x::<10>(&data);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
