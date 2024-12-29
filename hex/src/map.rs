//
// 
//
use std::ops::Index;

use crate::point::{Point,ORIGIN};
use crate::hex::Hex;


pub enum Shape {
    Rectangle,
    Hexagon,
    MegaHex
}

pub enum Orientation {
    Horizontal,
    Vertical
}

pub struct Row {
    row: Vec<Hex>
}

impl Row {
    fn New() -> Row {
	Row { row: vec!() }
    }

    fn push(&mut self, h: Hex) {
	self.row.push(h);
    }
}

impl Index<usize> for Row {
    type Output = Hex;

    fn index(&self, loc: usize) -> &Hex {
	&(self.row[loc])
    }

}

pub struct Map {
    pub shape: Shape,
    pub orientation: Orientation,
    pub size: Point,
    pub origin: Point,
    rows: Vec<Row>
}

impl Map {
    
    fn New(shape: Shape, orientation: Orientation, size: Point, origin: Point) -> Map {
	let mut rows: Vec<Row> = vec!();

	for i in 0..size.hx {
	    rows.push(Row::New());
	    for j in 0..size.hy {
		rows[i as usize].push(Hex::new (Point { hx: i + origin.hx, hy: j + origin.hy }));
	    }
	}
	
	Map { shape: shape, orientation: orientation, size: size, origin: origin, rows: rows }
    }
}

#[cfg(test)]

#[test]
fn test_map_new() {
    let m0 = Map::New(Shape::Rectangle, Orientation::Vertical, Point { hx: 5, hy: 6 }, ORIGIN);

    assert_eq!(m0.size.hx, 5);
}
