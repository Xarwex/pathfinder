use bevy::ecs::component::Component;
use bevy::utils::HashSet;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    x: isize,
    y: isize,
}

impl Block {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn is_adjacent(self, block: &Block) -> bool {
        [(self.x, block.x), (self.y, block.y)]
            .iter()
            .fold(false, |acc, (a, b)| acc ^ ((a - b).abs() == 1))
    }

    pub fn get_adjacent(self) -> HashSet<Block> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(x, y)| Block {
                x: self.x + x,
                y: self.y + y,
            })
            .collect()
    }
}
