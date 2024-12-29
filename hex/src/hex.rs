//
//
//
use crate::point::Point;

pub struct Hex {
    location: Point,
//    terrain: Vec<Terrain>,
//    contents: mut Vec<Item>,
//    occupants: mut Vec<Unit>	
}

impl Hex {
    pub fn new(location: Point) -> Hex {
	Hex { location: location }
    }
}
#[cfg(test)]

#[test]
fn test_hex() {
    let h1 = Hex { location: Point { hx: -3, hy: 14 } };

    assert_eq!( h1.location.hx, -3 );
}
