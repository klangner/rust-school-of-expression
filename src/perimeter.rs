
use std::f64::consts::PI;
use shapes::{Point, Shape, Float, distance_between};


pub trait HasPerimeter {
    fn perimeter(&self) -> Float;
}


impl HasPerimeter for Shape {
    fn perimeter(&self) -> Float {
        match self {
            &Shape::Rectangle {width: w, height: h} => 2.0*(w+h),
            &Shape::Ellipse {rx: x, ry: y} => ellipse_perimeter(x, y),
            &Shape::RtTriangle {width: w, height: h} => w+h+(w.powi(2) + h.powi(2)).sqrt(),
            &Shape::Polygon {points: ref ps} => polygon_perimeter(ps),
        }
    }
}

/// Calculate ellipse perimeter
fn ellipse_perimeter(a: Float, b: Float) -> Float {
    let h = (a-b).powi(2) / (a+b).powi(2);
    PI*(a+b) * (1.0 + 3.0*h/(10.0+(4.0-3.0*h).sqrt()))
}

/// Calculate polygon area as a sum of trapezoids
fn polygon_perimeter(ps: &Vec<Point>) -> Float {
    // Polygon need at least 3 vertices.
    if ps.len() < 3 { return 0.0 }
    let last_point = &ps[ps.len()-1];
    ps.iter().fold((0.0, last_point), |(sum, lp), p| {
        (sum + distance_between(lp, p) , p)
    }).0
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use shapes;

    #[test]
    fn square_test() {
        let s = shapes::square(100.0);
        assert!(s.perimeter() == 4.0*100.0);
    }

    #[test]
    fn triangle_test() {
        let s = shapes::rt_triangle(3.0, 4.0);
        assert!(s.perimeter() == 12.0);
    }

    #[test]
    fn ellipse_test() {
        let s = shapes::ellipse(5.0, 10.0);
        assert!(s.perimeter().floor() == 48.0);
    }
}