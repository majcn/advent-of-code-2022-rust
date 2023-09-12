use std::vec;

pub struct Grid<T> {
    arena: Vec<Node<T>>,
}

struct Node<T> {
    // idx: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Grid<T> {
    pub fn new(value: T) -> Self {
        Grid {
            arena: vec![Node {
                // idx: 0,
                value: value,
                parent: None,
                children: vec![],
            }],
        }
    }

    pub fn insert(&mut self, node: usize, value: T) -> usize {
        let new_node_idx = self.arena.len();

        let new_node = Node {
            // idx: new_node_idx,
            value: value,
            parent: Some(node),
            children: vec![],
        };

        self.arena.push(new_node);
        self.arena[node].children.push(new_node_idx);
        new_node_idx
    }

    pub fn get(&self, node: usize) -> &T {
        &self.arena[node].value
    }

    pub fn get_parent(&self, node: usize) -> Option<usize> {
        self.arena[node].parent
    }

    pub fn get_children(&self, node: usize) -> Vec<usize> {
        self.arena[node].children.clone()
    }

    pub fn has_children(&self, node: usize) -> bool {
        !self.arena[node].children.is_empty()
    }

    pub fn get_values(&self) -> Vec<(usize, &T)> {
        self.arena
            .iter()
            .enumerate()
            .map(|(i, node)| (i, &node.value))
            .collect()
    }
}
