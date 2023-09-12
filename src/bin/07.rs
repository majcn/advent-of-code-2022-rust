use advent_of_code::util::grid::ArenaTree;

enum NodeValueEnum {
    File(u32),
    Folder,
}

fn parse_data(input: &str) -> ArenaTree<NodeValueEnum> {
    let mut grid = ArenaTree::new(NodeValueEnum::Folder);
    let mut grid_current_idx = 0;

    for line in input.lines() {
        if line.starts_with("$ cd") {
            grid_current_idx = match line.as_bytes()[5] {
                b'/' => grid_current_idx,
                b'.' => grid.get_parent(grid_current_idx).unwrap(),
                _ => grid.insert(grid_current_idx, NodeValueEnum::Folder),
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let size = line
                .split_once(' ')
                .map(|(size, _)| size.parse::<u32>().unwrap())
                .unwrap();

            grid.insert(grid_current_idx, NodeValueEnum::File(size));
        }
    }

    grid
}

fn get_all_folders(grid: &ArenaTree<NodeValueEnum>) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            NodeValueEnum::Folder => Some(i),
            _ => None,
        })
        .collect()
}

fn calculate_size(grid: &ArenaTree<NodeValueEnum>, node: usize) -> u32 {
    match grid[node] {
        NodeValueEnum::File(v) => v,
        NodeValueEnum::Folder => grid
            .get_children(node)
            .into_iter()
            .map(|node| calculate_size(grid, node))
            .sum(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);
    let result = get_all_folders(&data)
        .into_iter()
        .map(|folder| calculate_size(&data, folder))
        .filter(|size| size < &100000)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const REQUIRED_DISK_SPACE: u32 = 30_000_000;

    let used_disk_space = calculate_size(&data, 0);
    let result = get_all_folders(&data)
        .into_iter()
        .map(|folder| calculate_size(&data, folder))
        .filter(|size| used_disk_space - size < TOTAL_DISK_SPACE - REQUIRED_DISK_SPACE)
        .min()
        .unwrap();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
