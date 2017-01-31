
use std::f64::consts::PI;
use std::f64;


/// Default type for coordinates
type Float = f64;

pub trait HasArea{
    fn area(&self) -> Float;
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rectangle {
        width: Float,
        height: Float,
    },

    Ellipse {
        rx: Float,
        ry: Float,
    },

    RtTriangle {
        width: Float,
        height: Float,
    },

    Polygon {
        points: Vec<Point>
    },
}

impl HasArea for Shape {
    fn area(&self) -> Float {
        match self {
            &Shape::Rectangle {width: w, height: h} => w*h,
            &Shape::Ellipse {rx: x, ry: y} => PI*x*y,
            &Shape::RtTriangle {width: w, height: h} => w*h/2.0,
            &Shape::Polygon {points: ref ps} => polygon_area(&ps),
        }
    }
}


/// Calculate area of trapezoid
/// The base of the trapezoid is X axis.
pub fn trapezoid_area(p1: &Point, p2: &Point) -> Float {
    let x = (p1.x - p2.x).abs();
    let y1 = f64::min(p1.y, p2.y);
    let y2 = (p1.y - p2.y).abs();
    x*(y1 + 0.5*y2)
}


/// Calculate polygon area as a sum of trapezoids
fn polygon_area(ps: &Vec<Point>) -> Float {
    // Polygon need at least 3 vertices.
    if ps.len() < 3 { return 0.0 }
    let last_point = &ps[ps.len()-1];
    ps.iter().fold((0.0, last_point), |(sum, lp), p| {
        if p.x > lp.x {
            (sum + trapezoid_area(lp, p) , p)
        } else {
            (sum - trapezoid_area(lp, p) , p)
        }
    }).0
}


pub fn point(x: Float, y: Float) -> Point {
    Point { x: x, y: y}
}

pub fn rect(width: Float, height: Float) -> Shape {
    Shape::Rectangle { width: width, height: height }
}

pub fn square(width: Float) -> Shape {
    Shape::Rectangle { width: width, height: width }
}

pub fn ellipse(r1: Float, r2: Float) -> Shape {
    Shape::Ellipse {rx: r1, ry: r2}
}

pub fn circle(r: Float) -> Shape {
    Shape::Ellipse {rx: r, ry: r}
}

pub fn rt_triangle(a: Float, b: Float) -> Shape {
    Shape::RtTriangle {width: a, height: b}
}

pub fn polygon(ps: Vec<Point>) -> Shape {
    Shape::Polygon {points: ps}
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_is_rect_test() {
        let r = rect(100.0, 100.0);
        let s = square(100.0);
        assert!(r == s)
    }

    #[test]
    fn rect_area_test() {
        let r = rect(100.0, 200.0);
        assert!(r.area() == 20000.0)
    }

    #[test]
    fn triangle_area_test() {
        let p1 = Point { x: 2.0, y: 5.0 };
        let p2 = Point { x: 2.0, y: 1.0 };
        let p3 = Point { x: 3.0, y: 1.0 };
        assert!(triangle_area(&p1, &p2, &p3).floor() == 2.0)
    }

    #[test]
    fn trapezoid_area_test() {
        let p1 = Point { x: 2.0, y: 5.0 };
        let p2 = Point { x: 4.0, y: 1.0 };
        assert!(trapezoid_area(&p1, &p2).floor() == 6.0)
    }

    #[test]
    fn polygon_area_test() {
        let p1 = Point { x: 10.0, y: 1.0 };
        let p2 = Point { x: 7.0, y: 5.0 };
        let p3 = Point { x: 10.0, y: 9.0 };
        let p4 = Point { x: 13.0, y: 5.0 };
        let poly = polygon(vec![p1, p2, p3, p4]);
        assert!(poly.area().floor() == 24.0)
    }
}