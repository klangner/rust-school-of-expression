
use piston_window::{Graphics, ellipse, polygon, rectangle, types, math};
use shapes::{Point, Shape};


pub fn draw_shape<G>(color: types::Color, pos: &Point, shape: &Shape, trans: math::Matrix2d,
                     g: &mut G) where G: Graphics
{
    match shape {
        &Shape::Rectangle {width: w, height: h} => {
            rectangle(color, [pos.x, pos.y, w, h], trans, g);
        },
        &Shape::Ellipse {rx: x, ry: y} => {
            ellipse(color, [pos.x, pos.y, x, y], trans, g);
        },
        &Shape::RtTriangle {width: w, height: h} => {
            let xs = [[pos.x, pos.y], [pos.x+w, pos.y], [pos.x, pos.y+h]];
            polygon(color, &xs, trans, g);
        },
        &Shape::Polygon {points: ref ps} => {
            let xs = ps.iter().map(|p| [p.x, p.y]).collect::<Vec<[f64; 2]>>();
            polygon(color, xs.as_slice(), trans, g);
        },
    }
    ;
}