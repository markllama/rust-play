//
/// This class defines Hex Point and the algebra for manipulating them
///
/// The algorithms are derived from the wonderful page by Redblob Games:
/// [Hexagonal Grids](https://www.redblobgames.com/grids/hexagons/)
///
/// Mathematically a hex map is a
/// [triangular lattice](https://en.wikipedia.org/wiki/Hexagonal_lattice).
/// Each node is directly connected to six other nodes in pairs on
/// three axes. A single node can be defined by its distance from the
/// origin on two axes. The location on the third axis is dependent
/// and can be derived from the other two values.

/// This model uses a slightly modified version of the
/// [Axial](https://www.redblobgames.com/grids/hexagons/#coordinates-axial)
/// coordinate system. All of the possible coordinate systems are equivalent
/// and so can be converted from one to the other if needed. 

/// Each hex location is defined by two integers, hx and hy
pub struct Point {
    hx: i32,
    hy: i32
}

/// Define a reference point for all others: (0, 0)
pub const ORIGIN:Point = Point { hx: 0, hy: 0 };

/// Define a unit vector in all 6 directions
pub const UNIT:[Point; 6] = [
    Point { hx: 0, hy: -1 },
    Point { hx: 1, hy: 0 },
    Point { hx: 1, hy: 1 },
    Point { hx: 0, hy: 1 },
    Point { hx: -1, hy: 0 },
    Point { hx: -1, hy: -1 }
];

impl Point {

    /// The third axis location is dependent on the other two:
    ///   hx = hy - hx
    pub fn hz(&self) -> i32 {
	self.hy - self.hx
    }

    /// Two hex points are equal when both elements are equal
    pub fn eq(&self, other: Point) -> bool {
	self.hx == other.hx && self.hy == other.hy
    }

    /// the sum of two points is just sum of both components
    pub fn add(&self, other: Point) -> Point {
	Point { hx: self.hx + other.hx, hy: self.hy + other.hy }
    }

    /// the diff of two points is just diff of both components
    pub fn sub(&self, other: Point) -> Point {
	Point { hx: self.hx - other.hx, hy: self.hy - other.hy }
    }
    
    /// Distance is the maximum of the differences of the axes, but
    /// because they are related by subtraction you can
    /// just add the three and divide by 2.
    /// See: [Axial Distance](https://www.redblobgames.com/grids/hexagons/#distances-axial)
    pub fn distance(&self, other: Point) -> i32 {
	let diff = self.sub(other);
	(diff.hx.abs() + diff.hy.abs() + (diff.hx + diff.hy).abs()) / 2
    }

    // pub fn line(&self, other: Point -> Vec<Point> {
    // 	// How long is the line?
    // 	let dist = self.distance(other);

    // 	// create an empty vector for the hexes in the line
    // 	let line Vec<Point> = vec![];

    // 	// interpolate for each line
    // 	for i in 0..(dist-1) {
    // 	    // get the f32 interpolated location

    // 	    // reduce back to int

	    
    // 	}


	
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        assert_eq!(ORIGIN.hx, 0);
        assert_eq!(ORIGIN.hy, 0);
	assert_eq!(ORIGIN.hz(), 0);
    }

    // test_unit()

    // test_hz()

    // test_eq()

    // test_add()

    // test_sub()

    // test_distance()

    // test_line()
}
