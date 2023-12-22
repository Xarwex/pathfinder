pub type Coordinate = isize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Point {
    pub fn new_from_tuple((x, y): (Coordinate, Coordinate)) -> Self {
        Self { x, y }
    }
}
