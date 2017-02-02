
use geometry::{Float, Vector};
use shapes;


#[derive(Debug, PartialEq)]
pub enum Region {
    Shape {
        shape: shapes::Shape
    },

    Translation {
        vector: Vector,
        region: Box<Region>
    },

    Scale {
        vector: Vector,
        region: Box<Region>
    },

    Inverse {
        region: Box<Region>
    },

    Union {
        region1: Box<Region>,
        region2: Box<Region>
    },

    Intersection {
        region1: Box<Region>,
        region2: Box<Region>
    },

    Empty
}


impl shapes::HasPoints for Region {
    fn is_in(&self, x: Float, y: Float) -> bool {
        match self {
            &Region::Shape {shape: ref s} => {
                s.is_in(x, y)
            },
            &Region::Translation {vector: ref v, region: ref r} => {
                r.is_in(x-v.x, y-v.y)
            },
            &Region::Scale {vector: ref v, region: ref r} => {
                r.is_in(x/v.x, y/v.y)
            },
            &Region::Inverse {region: ref r} => {
                !r.is_in(x, y)
            },
            &Region::Union {region1: ref r1, region2: ref r2} => {
                r1.is_in(x, y) || r2.is_in(x, y)
            },
            &Region::Intersection {region1: ref r1, region2: ref r2} => {
                r1.is_in(x, y) && r2.is_in(x, y)
            },
            &Region::Empty => false
        }
    }
}



/// Create Region from shape
pub fn region(s: shapes::Shape) -> Region {
    Region::Shape { shape: s }
}

/// Translate region
pub fn translate(v: Vector, r: Region) -> Region {
    Region::Translation {vector: v, region: Box::new(r)}
}

/// Scale region
pub fn scale(v: Vector, r: Region) -> Region {
    Region::Scale {vector: v, region: Box::new(r)}
}

/// Inverse region
pub fn inverse(r: Region) -> Region {
    Region::Inverse {region: Box::new(r)}
}

pub fn union(r1: Region, r2: Region) -> Region {
    Region::Union {region1: Box::new(r1), region2: Box::new(r2)}
}

pub fn intersection(r1: Region, r2: Region) -> Region {
    Region::Intersection {region1: Box::new(r1), region2: Box::new(r2)}
}

/// Create empty region
pub fn empty() -> Region {
    Region::Empty
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use shapes;
    use shapes::HasPoints;
    use geometry::{vector};


    #[test]
    fn inside_shape_test() {
        let s = region(shapes::rt_triangle(3.0, 4.0));
        assert!(s.is_in(1.0, 2.0));
        assert!(!s.is_in(1.0, 12.0));
    }

    #[test]
    fn inside_empty_test() {
        let s = empty();
        assert!(!s.is_in(1.0, 12.0));
    }

    #[test]
    fn inside_translate_test() {
        let s = region(shapes::rt_triangle(3.0, 4.0));
        let t = translate(vector(5.0, 5.0), s);
        assert!(t.is_in(6.0, 7.0));
        assert!(!t.is_in(6.0, 17.0));
    }

    #[test]
    fn inside_scale_test() {
        let s = region(shapes::rect(3.0, 4.0));
        let t = scale(vector(5.0, 5.0), s);
        assert!(t.is_in(12.0, 16.0));
        assert!(!t.is_in(26.0, 16.0));
    }

    #[test]
    fn inside_inverse_test() {
        let s = region(shapes::rect(3.0, 4.0));
        let t = inverse(s);
        assert!(t.is_in(12.0, 16.0));
        assert!(!t.is_in(2.0, 1.0));
    }

    #[test]
    fn inside_union_test() {
        let s1 = region(shapes::rect(3.0, 4.0));
        let s2 = region(shapes::rect(13.0, 14.0));
        let t = union(s1, s2);
        assert!(t.is_in(12.0, 13.0));
        assert!(!t.is_in(12.0, 16.0));
    }

    #[test]
    fn inside_intersection_test() {
        let s1 = region(shapes::rect(3.0, 4.0));
        let s2 = region(shapes::rect(13.0, 14.0));
        let t = intersection(s1, s2);
        assert!(t.is_in(2.0, 1.0));
        assert!(!t.is_in(12.0, 13.0));
    }
}