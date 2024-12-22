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
    pub hx: i32,
    pub hy: i32
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

    pub fn neighbor(&self, direction: i32) -> Point {
	*self + UNIT[(direction.rem_euclid(6)) as usize]
    }
    
    /// Get a vector pointing in the opposite direction
    pub fn invert(&self) -> Point {
	Point { hx: self.hx * -1, hy: self.hy * -1 }
    }

    /// reflect around one axis. Invert the other two axes
    pub fn reflect(&self, axis: i32 ) -> Point {
	match axis.rem_euclid(3) {
	    0 => Point { hx: self.hx, hy: self.hz() },
	    1 => Point { hx: self.hz(), hy: self.hy },
	    2 => Point { hx: self.hy, hy: self.hx },
	    _ => ORIGIN
	}
    }

    pub fn reflect_hx(&self) -> Point {
	Point { hx: self.hx, hy: self.hz() }
    }

    pub fn reflect_hy(&self) -> Point {
	Point { hx: self.hz(), hy: self.hy }	
    }

    pub fn reflect_hz(&self) -> Point {
	Point { hx: self.hy, hy:  self.hx }
    }
    
    pub fn rotate(&self, hextant: i32) -> Point {
	// reduce the rotation to one full cycle at most
	let rot = hextant.rem_euclid(3) as usize;
	println!("hextant: {}, rot: {}", hextant, rot);
	let invert = if hextant.rem_euclid(2) == 0 { 1 } else {-1 };
	let ring = [self.hx, self.hy, self.hz(), self.hx, self.hy];
	Point { hx: ring[rot] * invert, hy: ring[rot+1] * invert }
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

    // https://www.redblobgames.com/grids/hexagons/#range-coordinate
    pub fn region(&self, dist: i32) -> Vec<Point> {
	let mut range: Vec<Point> = vec!();

	for hx in -dist..dist+1 {
	    for hy in (-dist).max(-hx - dist)..(dist.min(-hx+dist))+1 {
		range.push(*self + Point { hx: hx, hy: hy });
	    }
	}
	range
    }

    pub fn ring(&self, radius: i32) -> Vec<Point> {

	if radius == 0 {
	    return vec!(*self);
	}

	let mut next = UNIT[4] * radius.abs();
	let mut ring = vec!();
	
	for hextant in 0..6 {
	    for _step in 0..radius {
		ring.push(next);
		next = next.neighbor(hextant); 
	    }
	}
	ring
    }
}
