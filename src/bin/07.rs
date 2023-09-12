use advent_of_code::util::grid::Grid;

enum NodeValueEnum {
    File(u32),
    Folder,
}

fn parse_data(input: &str) -> Grid<NodeValueEnum> {
    let mut grid = Grid::new(NodeValueEnum::Folder);
    let mut grid_current_idx = 0;

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let folder = &line[5..];
            match folder.as_bytes()[0] {
                b'/' => {}
                b'.' => {
                    grid_current_idx = grid.get_parent(grid_current_idx).unwrap();
                }
                _ => {
                    grid_current_idx = grid.insert(grid_current_idx, NodeValueEnum::Folder);
                }
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

fn get_all_folders(grid: &Grid<NodeValueEnum>) -> Vec<usize> {
    grid.get_values()
        .into_iter()
        .filter_map(|x| match x.1 {
            NodeValueEnum::Folder => Some(x.0),
            _ => None,
        })
        .collect()
}

fn calculate_size(grid: &Grid<NodeValueEnum>, node: usize) -> u32 {
    match grid.get(node) {
        NodeValueEnum::File(v) => *v,
        NodeValueEnum::Folder => {
            let mut folder_size = 0;
            for c in grid.get_children(node) {
                folder_size += calculate_size(&grid, c)
            }
            folder_size
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);
    let result = get_all_folders(&grid)
        .into_iter()
        .map(|folder| calculate_size(&grid, folder))
        .filter(|size| size < &100000)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let total_disk_space = 70_000_000;
    let required_disk_space = 30_000_000;

    let used_disk_space = calculate_size(&grid, 0);
    let result = get_all_folders(&grid)
        .into_iter()
        .map(|folder| calculate_size(&grid, folder))
        .filter(|size| used_disk_space - size < total_disk_space - required_disk_space)
        .min();

    result
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
