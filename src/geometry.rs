
use std::f64;


/// Default type for coordinates
pub type Float = f64;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}


#[derive(Debug, PartialEq)]
pub struct Vector {
    dx: Float,
    dy: Float
}


/// Calculate distance between 2 points
pub fn distance_between(p1: &Point, p2: &Point) -> Float {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn point(x: Float, y: Float) -> Point {
    Point { x: x, y: y}
}

//pub fn vector(x: Float, y: Float) -> Vector {
//    Vector { dx: x, dy: y}
//}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_test() {
        let p1 = Point { x: 2.0, y: 5.0 };
        let p2 = Point { x: 2.0, y: 1.0 };
        assert!(distance_between(&p1, &p2).floor() == 4.0)
    }
}