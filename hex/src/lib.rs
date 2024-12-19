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

use std::ops::{Add, Sub, Mul};

// this pulled in a LOT of stuff and could be a tuple of f32 for one internal call.
// But Learning
use geo::geometry::Coord;

/// Each hex location is defined by two integers, hx and hy
#[derive(Clone, Copy, Debug, PartialEq)] 
pub struct Point {
    hx: i32,
    hy: i32
}

/// Define a reference point for all others: (0, 0)
pub const ORIGIN:Point = Point { hx: 0, hy: 0 };

/// Define a unit vector in all 6 directions
/// 0 and 3 are "on the hx axis" and 1,4 are "on the hy axis"
/// That means 2,5 are "on the hz axis" because hy - hx == 0
pub const UNIT:[Point; 6] = [
    Point { hx: 0, hy: -1 },    // direction 0
    Point { hx: 1, hy: -1 },     // direction 1
    Point { hx: 1, hy: 0 },     // direction 2
    Point { hx: 0, hy: 1 },     // direction 3
    Point { hx: -1, hy: 1 },    // direction 4
    Point { hx: -1, hy: 0 }    // direction 5
];

impl Add for Point {
    type Output = Self;
    
    /// the sum of two points is just sum of both components
    fn add(self, other: Self) -> Self {
	Self { hx: self.hx + other.hx, hy: self.hy + other.hy }
    }
}

impl Sub for Point {
    type Output = Self;
    /// the diff of two points is just diff of both components
    fn sub(self, rhs: Self) -> Self {
	Self { hx: self.hx - rhs.hx, hy: self.hy - rhs.hy }
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    /// Multiply a hex vector by a scalar
    fn mul(self, rhs: i32) -> Self {
	Self { hx: self.hx * rhs, hy: self.hy * rhs }
    }
}

impl Point {

    /// The third axis location is dependent on the other two:
    ///   hx = hy - hx
    pub fn hz(&self) -> i32 {
	self.hy - self.hx
    }

    /// Get a vector pointing in the opposite direction
    pub fn invert(&self) -> Point {
	Point { hx: self.hx * -1, hy: self.hy * -1 }
    }
    
    /// Distance is the maximum of the differences of the axes, but
    /// because they are related by subtraction you can
    /// just add the three and divide by 2.
    /// See: [Axial Distance](https://www.redblobgames.com/grids/hexagons/#distances-axial)
    pub fn distance(&self, other: &Point) -> i32 {
	let diff = *self - *other;
	(diff.hx.abs() + (diff.hx + diff.hy).abs() + diff.hy.abs()) / 2
    }

    // find the delta for a single hex in a line
    // (dx / n+1, dy / n+1)
    fn slope(&self, other: &Point) -> Coord<f32> {
	let length = self.distance(other) as f32;
	let diff = *other - *self;
	Coord { x: diff.hx as f32 / length, y: diff.hy as f32 / length }
    }

    // find a hex along the line
    fn interpolate(&self, other: &Point, step: i32) -> Point {
	let slope = self.slope(other);
	Point {
	    hx: ((self.hx as f32 + (step as f32 * slope.x).round()) as i32),
	    hy: ((self.hy as f32 + (step as f32 * slope.y).round()) as i32)
	}
    }

    pub fn line(&self, other: &Point) -> Vec<Point> {	
	(0..self.distance(other)+1).map( | i |
	    self.interpolate(&other, i)
	).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
	let h1 = Point { hx: 3, hy: -4 };
	let h2 = Point { hx: 3, hy: -4 };
	let h3 = Point { hx: -2, hy: 12 };
	assert_eq!(h1, h2);
	assert_eq!(h1, Point { hx: 3, hy: -4});
	assert_eq!(h3, Point { hx: -2, hy: 12 });
	assert_ne!(h1, h3);
    }
    
    #[test]
    fn test_origin() {
	// all three dimensions should be zero	
        assert_eq!(ORIGIN.hx, 0);    
        assert_eq!(ORIGIN.hy, 0);
	assert_eq!(ORIGIN.hz(), 0);
	assert_eq!(ORIGIN, Point { hx: 0, hy: 0 });
	assert_eq!(Point { hx: 0, hy: 0 }, ORIGIN);
    }

    #[test]
    fn test_unit() {
	// Each of the 6 unit vectors represents a 1 hex move in one of the "cardinal"
	// directions.

	// unit vectors 0 and 3 are on the hx axis
	assert_eq!(&UNIT[0], &Point { hx: 0, hy: -1 });
	assert_eq!(&UNIT[3], &Point { hx: 0, hy: 1 });

	// unit vectors 1 and 4 are on the hy axis
	assert_eq!(&UNIT[1], &Point { hx: 1, hy: -1 });
	assert_eq!(&UNIT[4], &Point { hx: -1, hy: 1 });

	// unit vectors 2 and 5 are on the hx axis (hy - hx = 0)
	assert_eq!(&UNIT[2], &Point { hx: 1, hy: 0 });
	assert_eq!(&UNIT[5], &Point { hx: -1, hy: 0 });
    }

    // test_hz()
    #[test]
    fn test_hz() {
	// the third axis is dependent on the other two. It is hy = hx
	assert_eq!(0, Point { hx: 0, hy: 0 }.hz() );
	assert_eq!(1, Point { hx: 0, hy: 1 }.hz() );
	assert_eq!(4, Point { hx: -2, hy: 2 }.hz() );
	assert_eq!(-8, Point { hx: 9, hy: 1 }.hz() );
    }

    // test_add()
    #[test]
    fn test_add() {
	// hex arithmetic functions match cartesian ones
	// Simply add or subtract the components
	assert_eq!(Point { hx: 4, hy: 7} + Point { hx: -4, hy: -7 }, ORIGIN);
	assert_eq!(Point { hx: -3, hy: 4} + Point { hx: 3, hy: -4 }, ORIGIN);
    }

    // test_sub()
    #[test]
    fn test_sub() {
	// hex arithmetic functions match cartesian ones
	// Simply add or subtract the components
	assert_eq!(Point { hx: 4, hy: 7} - Point { hx: 4, hy: 7 }, ORIGIN);
	assert_eq!(Point { hx: -3, hy: 4} - Point { hx: -3, hy: 4 }, ORIGIN);
    }

    // test_mul()
    #[test]
    fn test_mul() {
	// hex arithmetic functions match cartesian ones
	// Simply add or subtract the components
	assert_eq!(Point { hx: 4, hy: 7} * 3, Point { hx: 12, hy: 21} );
	assert_eq!(Point { hx: -3, hy: 4} * -2, Point { hx: 6, hy: -8} );
    }

    // test_distance()
    #[test]
    fn test_distance() {
	// the distance between two hex points is the max of the absolute value
	// of the three components.
	// for unit in &UNIT {
	//   assert_eq!(ORIGIN.distance(unit), 1i32);
	// }
	assert_eq!(ORIGIN.distance(&UNIT[0]), 1);
	assert_eq!(ORIGIN.distance(&UNIT[1]), 1);
	assert_eq!(ORIGIN.distance(&UNIT[2]), 1);
	assert_eq!(ORIGIN.distance(&UNIT[3]), 1);
	assert_eq!(ORIGIN.distance(&UNIT[4]), 1);
	assert_eq!(ORIGIN.distance(&UNIT[5]), 1);
    }

    #[test]
    fn test_slope() {
	// Find the delta for a single hex so that I can

	let ends = ( Point { hx: 4, hy: 9 }, Point { hx: 7, hy: -2 });
	let slope1 = ends.0.slope(&ends.1);

	assert_eq!(slope1.x, (7.0 - 4.0) / 11.0);
	assert_eq!(slope1.y, (-2.0 - 9.0) / 11.0);
    }

    
    #[test]
    fn test_interpolate() {
	// check that each point along a line is calculated correctly
	// first line is straight on the hx axis

	// test interpolating the origin and units
	for h in 0..5 {
	    for i in 0..1 {
		let r0 = ORIGIN.interpolate(&UNIT[h], i);
		// println!("{i} : {:#?}", r0);
		assert_eq!(r0, ORIGIN);
		let r2 = UNIT[h].interpolate(&ORIGIN, i);
		// println!("{i} : {:#?}", r2);
		assert_eq!(r2, UNIT[h]);
	    }
	}
    
	// check long lines along the axes from the origin
	let pair1 = ( ORIGIN, Point { hx: 6, hy: 0} );
	for i in 0..6 {
	    let r = pair1.0.interpolate(&pair1.1,  i);
	    // println!("{i} : {:#?}", r);
	    assert_eq!(r, Point { hx: i, hy: 0 });
	}
	for i in 6..0 {
	    let r = pair1.1.interpolate(&pair1.0,  i);
	    // println!("{i} : {:#?}", r);
	    assert_eq!(r, Point { hx: i, hy: 0 });
	}

	let pair2 = ( ORIGIN, Point { hx: 0, hy: 6} );
	for i in 0..6 {
	    let r = pair2.0.interpolate(&pair2.1,  i);
	    // println!("{i} : {:#?}", r);
	    assert_eq!(r, Point { hx: 0, hy: i });
	}
	for i in 6..0 {
	    let r = pair2.1.interpolate(&pair2.0,  i);
	    // println!("{i} : {:#?}", r);
	    assert_eq!(r, Point { hx: i, hy: 0 });
	}

	// check on the axes through the origin
	let radius = 5;
	let n_hexes = radius * 2 + 1;
	let pair = ( UNIT[0] * radius, UNIT[0] * -radius );
	println!("Test Values: {:?}", pair);
	for i in 0..n_hexes+1 {
	    let r = pair.0.interpolate(&pair.1, i);
	    //pair.0 + ( ORIGIN + (UNIT[0] * (i - radius)));
	    println!("{i} : {:#?}", r);	
	    assert_eq!(r, ((UNIT[0] * radius) + (UNIT[0].invert() * i)));
	}
    }
}
