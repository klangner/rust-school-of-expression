
trait HasArea{
    fn area(&self) -> f64;
}

#[derive(Debug)]
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

//impl HasArea for Rectangle {
//    fn area(&self) -> f64 {
//        self.width * self.height
//    }
//}


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