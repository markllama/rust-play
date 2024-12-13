pub struct Point {
    hx: i32,
    hy: i32
}

impl Point {

    pub fn hz(&self) -> i32 {
	self.hy - self.hx
    }
    
    pub fn origin() -> Point {
	Point { hx: 0, hy: 0 }
    }
    
}
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
