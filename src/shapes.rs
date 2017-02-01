
use std::f64::consts::PI;
use std::f64;
use geometry::{Float, Point, point};


pub trait HasArea {
    fn area(&self) -> Float;
}

pub trait HasPoints {
    fn is_in(&self, x: Float, y: Float) -> bool;
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


impl HasPoints for Shape {
    fn is_in(&self, x: Float, y: Float) -> bool {
        match self {
            &Shape::Rectangle {width: w, height: h} => {
                x >= 0.0 && x <= w && y >= 0.0 && y <= h
            },
            &Shape::Ellipse {rx: r1, ry: r2} => {
                (x/r1).powi(2) + (y/r2).powi(2) <= 1.0
            },
            &Shape::RtTriangle {width: w, height: h} => {
                let ps = vec![point(0.0, 0.0), point(w, 0.0), point(0.0, h)];
                is_in_polygon(&ps, x, y)
            },
            &Shape::Polygon {points: ref ps} => is_in_polygon(&ps, x, y),
        }
    }
}


/// Calculate polygon area as a sum of trapezoids
fn is_in_polygon(ps: &Vec<Point>, x: Float, y: Float) -> bool {
    // Polygon need at least 3 vertices.
    if ps.len() < 3 { return false }
    let point = point(x, y);
    let last_point = &ps[ps.len()-1];
    ps.iter().fold((true, last_point), |(acc, lp), p| {
        (acc && is_right_of_line(&point, &lp, &p), p)
    }).0
}

fn is_right_of_line(p: &Point, a: &Point, b: &Point) -> bool {
    let (s, t) = (p.x - a.x, p.y - a.y);
    let (u, v) = (p.x - b.x, p.y - b.y);
    s*v >= t*u
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
    use geometry::{Point};

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
        assert!(rt_triangle(3.0, 4.0).area().floor() == 6.0)
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

    #[test]
    fn inside_rect_test() {
        let r = rect(100.0, 100.0);
        assert!(r.is_in(50.0, 23.0));
        assert!(!r.is_in(150.0, 23.0));
    }

    #[test]
    fn inside_ellipse_test() {
        let r = ellipse(10.0, 5.0);
        assert!(r.is_in(5.0, -3.0));
        assert!(!r.is_in(150.0, 2.0));
    }

    #[test]
    fn inside_polygon_test() {
        let p1 = Point { x: 5.0, y: 1.0 };
        let p2 = Point { x: 1.0, y: 5.0 };
        let p3 = Point { x: 5.0, y: 9.0 };
        let p4 = Point { x: 9.0, y: 5.0 };
        let poly = polygon(vec![p1, p2, p3, p4]);
        assert!(poly.is_in(5.0, 5.0));
        assert!(!poly.is_in(150.0, 6.0));
    }

    #[test]
    fn inside_triangle_test() {
        let s = rt_triangle(3.0, 4.0);
        assert!(s.is_in(1.0, 2.0));
        assert!(!s.is_in(1.0, 12.0));
    }

}