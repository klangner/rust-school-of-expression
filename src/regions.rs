
use geometry::{Float, Point, Vector};
use shapes;


#[derive(Debug, PartialEq)]
pub enum Region {
    Shape {
        shape: shapes::Shape
    },

    Translation {
        v: Vector,
        region: Region
    },

    Scale {
        v: Vector,
        region: Region
    },

    Inverse {
        region: Region
    },

    Union {
        region1: Region,
        region2: Region
    },

    Intersection {
        region1: Region,
        region2: Region
    },

    Empty
}


/// Create Region from shape
pub fn region(s: shapes::Shape) -> Region {
    Shape { shape: s }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn square_is_rect_test() {
//        let r = rect(100.0, 100.0);
//        let s = square(100.0);
//        assert!(r == s)
//    }
}