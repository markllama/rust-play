
 
#[cfg(test)]
mod tests {
    use hex::point::*;


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

    // test_neighbor
    #[test]
    fn test_neighbor() {
	for i in 0..6 {
	    assert_eq!(ORIGIN.neighbor(i), UNIT[i as usize])
	}
    }
    
    // test_invert()

    // test_rotate()
    #[test]
    fn test_rotate() {
	let first = Point { hx: 1, hy: -4 };

	assert_eq!( first, first.rotate(0));
	assert_eq!( first, first.rotate(6));
	assert_eq!( first, first.rotate(-6));
	
	assert_eq!( Point { hx: first.hy * -1, hy: first.hz() * -1 }, first.rotate(1));
	assert_eq!( Point { hx: first.hy * -1, hy: first.hz() * -1 }, first.rotate(7));
	assert_eq!( Point { hx: first.hz() * -1, hy: first.hx * -1 }, first.rotate(-1));
	assert_eq!( Point { hx: first.hz() * -1, hy: first.hx * -1 }, first.rotate(-7));

	assert_eq!( Point { hx: first.hz(), hy: first.hx }, first.rotate(2));
	assert_eq!( Point { hx: first.hx * -1, hy: first.hy * -1 }, first.rotate(3));
	assert_eq!( first.invert(), first.rotate(3));
	assert_eq!( first.invert(), first.rotate(-3));

	assert_eq!( Point { hx: first.hz() * -1, hy: first.hx * -1 }, first.rotate(5));

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

    // #[test]
    // fn test_slope() {
    // 	// Find the delta for a single hex so that I can

    // 	let ends = ( Point { hx: 4, hy: 9 }, Point { hx: 7, hy: -2 });
    // 	let slope1 = ends.0.slope(&ends.1);

    // 	assert_eq!(slope1.x, (7.0 - 4.0) / 11.0);
    // 	assert_eq!(slope1.y, (-2.0 - 9.0) / 11.0);
    // }

    
    // #[test]
    // fn test_interpolate() {
    // 	// check that each point along a line is calculated correctly
    // 	// first line is straight on the hx axis

    // 	// test interpolating the origin and units
    // 	for h in 0..5 {
    // 	    for i in 0..1 {
    // 		let r0 = ORIGIN.interpolate(&UNIT[h], i);
    // 		// println!("{i} : {:#?}", r0);
    // 		assert_eq!(r0, ORIGIN);
    // 		let r2 = UNIT[h].interpolate(&ORIGIN, i);
    // 		// println!("{i} : {:#?}", r2);
    // 		assert_eq!(r2, UNIT[h]);
    // 	    }
    // 	}
    
    // 	// check long lines along the axes from the origin
    // 	let pair1 = ( ORIGIN, Point { hx: 6, hy: 0} );
    // 	for i in 0..6 {
    // 	    let r = pair1.0.interpolate(&pair1.1,  i);
    // 	    // println!("{i} : {:#?}", r);
    // 	    assert_eq!(r, Point { hx: i, hy: 0 });
    // 	}
    // 	for i in 6..0 {
    // 	    let r = pair1.1.interpolate(&pair1.0,  i);
    // 	    // println!("{i} : {:#?}", r);
    // 	    assert_eq!(r, Point { hx: i, hy: 0 });
    // 	}

    // 	let pair2 = ( ORIGIN, Point { hx: 0, hy: 6} );
    // 	for i in 0..6 {
    // 	    let r = pair2.0.interpolate(&pair2.1,  i);
    // 	    // println!("{i} : {:#?}", r);
    // 	    assert_eq!(r, Point { hx: 0, hy: i });
    // 	}
    // 	for i in 6..0 {
    // 	    let r = pair2.1.interpolate(&pair2.0,  i);
    // 	    // println!("{i} : {:#?}", r);
    // 	    assert_eq!(r, Point { hx: i, hy: 0 });
    // 	}

    // 	// check on the axes through the origin
    // 	let radius = 5;
    // 	let n_hexes = radius * 2 + 1;
    // 	let pair = ( UNIT[0] * radius, UNIT[0] * -radius );
    // 	println!("Test Values: {:?}", pair);
    // 	for i in 0..n_hexes+1 {
    // 	    let r = pair.0.interpolate(&pair.1, i);
    // 	    //pair.0 + ( ORIGIN + (UNIT[0] * (i - radius)));
    // 	    println!("{i} : {:#?}", r);	
    // 	    assert_eq!(r, ((UNIT[0] * radius) + (UNIT[0].invert() * i)));
    // 	}
    // }

    #[test]
    fn test_line() {
	// test interpolating the origin and units
	for h in 0..5 {
	    let expect: Vec<Point> = vec!(ORIGIN, UNIT[h]);
	    let actual = ORIGIN.line(&UNIT[h]);
	    assert_eq!(expect, actual);
	}

	// test lines through the origin along the axes of the unit vectors
	for h in 0..5 {
	    let expect: Vec<Point> = vec!(UNIT[h], ORIGIN, UNIT[(h + 3) % 6]);
	    let actual = UNIT[h].line(&UNIT[(h + 3) % 6]);
	    assert_eq!(expect, actual);
	}

	// test longer lines through the origin
	for h in 0..5 {
	    let expect: Vec<Point> = (0..11).map(
		{
		    | i | UNIT[h] * 5 + UNIT[(h + 3) % 6] * i
		}
	    ).collect();
	    let actual = (UNIT[h] * 5).line(&(UNIT[(h + 3) % 6] * 5));
	    assert_eq!(expect, actual);
	}


	// test each of the rings around the origin
// 	for radius in 1..6 {
// 	    // test lines in each arc
// 	    for hextant in 0..6 {
// 		for step in 0..radius+1 {
// 		    let end = Point { hx: radius, hy: -radius + step }.rotate(hextant);
// 		    //
// 		    // let expected = 
// 		    //
// 		    let actual = ORIGIN.line(&end);
// 		    println!("radius: {} hextant: {} step: {}, end: {:?}\nactual {:?} :", radius, hextant, step, end, actual)
// //		    println!("Step: {}, Hextant: {}, Radius {}", step, hextant, radius);
// 		}
// 	    }
	    
// 	}
// 	assert!(false)
	
    }

    #[test]
    fn test_region() {

	let r0 = ORIGIN.region(0);
	assert_eq!(1, r0.len());

	let r1 = ORIGIN.region(1);
	assert_eq!(7, r1.len());

	let r2 = ORIGIN.region(2);
	assert_eq!(19, r2.len());

	let r3 = ORIGIN.region(3);
	assert_eq!(37, r3.len());

	let r4 = UNIT[0].region(2);
	assert_eq!(19, r4.len());

	let r5 = Point { hx: 14, hy: -3 }.region(2);
	assert_eq!(19, r5.len());

    }

    #[test]
    fn test_ring() {

	// check single hex
	assert_eq!(ORIGIN.ring(0), vec!(ORIGIN));
	assert_eq!(Point { hx: -3, hy: 4 }.ring(0), vec!(Point {hx: -3, hy: 4 }));

	assert_eq!(ORIGIN.ring(1), vec!(UNIT[4], UNIT[5], UNIT[0], UNIT[1], UNIT[2], UNIT[3]));
	
    }

    #[test]
    fn test_spiral() {
	assert_eq!(ORIGIN.spiral(0), vec!(ORIGIN));
	assert_eq!(Point { hx: -3, hy: 4 }.spiral(0), vec!(Point {hx: -3, hy: 4 }));

	assert_eq!(ORIGIN.spiral(1).len(), 7);
	assert_eq!(ORIGIN.spiral(2).len(), 19);

	let mut spiral = ORIGIN.spiral(0);
	spiral.extend(ORIGIN.ring(1));
	assert_eq!(ORIGIN.spiral(1), spiral);
	spiral.extend(ORIGIN.ring(2));
	assert_eq!(ORIGIN.spiral(2), spiral);
	

    }
}
