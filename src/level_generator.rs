type Coordinate = u8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: Coordinate,
    y: Coordinate,
}

enum BlockType {
    Mirror,
    Empty,
    Blocking,
}

struct Level {
    grid: Vec<Vec<BlockType>>,
    starting_point: Point,
    finishing_point: Point,
}

struct LevelData {
    height: u8,
    width: u8,
    starting_point: Point,
    finishing_point: Point,
}

impl LevelData {
    fn generate_level(&self) -> Level {
        let current_node = self.starting_point;
        while current_node != self.finishing_point {
            // compute next node
            todo!()
        }
        todo!()
    }
}
