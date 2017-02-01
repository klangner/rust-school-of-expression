extern crate piston_window;

mod shapes;
mod draw;
mod perimeter;
mod geometry;
use piston_window::*;


fn main() {
    let model = build_model();
    let mut window: PistonWindow =
    WindowSettings::new("Hello Piston!", [800, 600])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            let color = [0.3, 0.7, 0.6, 1.0];
            for item in &model {
                draw::draw_shape(color, &item.0, &item.1, c.transform, g);
            }
        });
    }
}


fn build_model() -> Vec<(geometry::Point, shapes::Shape)> {
    let p1 = geometry::point(10.0, 100.0);
    let p2 = geometry::point(70.0, 50.0);
    let p3 = geometry::point(100.0, 0.0);
    let p4 = geometry::point(30.0, 50.0);

    vec![ (geometry::point(10.0, 10.0), shapes::rect(100.0, 200.0)),
          (geometry::point(150.0, 10.0), shapes::square(100.0)),
          (geometry::point(10.0, 250.0), shapes::ellipse(100.0, 200.0)),
          (geometry::point(150.0, 150.0), shapes::circle(100.0)),
          (geometry::point(300.0, 300.0), shapes::rt_triangle(100.0, 200.0)),
          (geometry::point(300.0, 100.0), shapes::polygon(vec![p1, p2, p3, p4]))]
}