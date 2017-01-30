
trait HasArea{
    fn area(&self) -> f64;
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rectangle {
        width: f64,
        height: f64,
    },

    Ellipse {
        rx: f64,
        ry: f64,
    },

    RtTriangle {
        width: f64,
        height: f64,
    },

    Polygon {
        points: Vec<f64>,
    },
}

impl HasArea for Shape {
    fn area(&self) -> f64 {
        match self {
            &Shape::Rectangle {width: w, height: h} => w*h,
            _ => 0.0,
        }
    }
}


pub fn rect(width: f64, height: f64) -> Shape {
    Shape::Rectangle { width: width, height: height }
}

pub fn square(width: f64) -> Shape {
    Shape::Rectangle { width: width, height: width }
}

pub fn ellipse(r1: f64, r2: f64) -> Shape {
    Shape::Ellipse {rx: r1, ry: r2}
}

pub fn circle(r: f64) -> Shape {
    Shape::Ellipse {rx: r, ry: r}
}

pub fn rt_triangle(a: f64, b: f64) -> Shape {
    Shape::RtTriangle {width: a, height: b}
}

pub fn polygon(ps: Vec<f64>) -> Shape {
    Shape::Polygon {points: ps}
}


/// Some unit tests

#[test]
fn square_is_rect() {
    let r = rect(100.0, 100.0);
    let s = square(100.0);
    assert!(r == s)
}


#[test]
fn rect_area() {
    let r = rect(100.0, 200.0);
    assert!(r.area() == 20000.0)
}
