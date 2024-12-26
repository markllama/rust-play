//
// 
//
use crate::point::Point;

pub enum Shape {
    Rectangle,
    Hexagon,
    MetaHex
}

pub enum Orientation {
    Horizontal,
    Vertical
}

pub struct Map {
    shape: Shape,
    orientation: Orientation,
    size: Point    
}
