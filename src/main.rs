extern crate piston_window;

mod shapes;
mod draw;
use piston_window::*;


fn main() {
    let model = build_model();
    let mut window: PistonWindow =
    WindowSettings::new("Hello Piston!", [800, 600])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            let red = [1.0, 0.0, 0.0, 1.0];
            for item in &model {
                draw::draw_shape(red, &item.0, &item.1, c.transform, g);
            }
        });
    }
}


fn build_model() -> Vec<(shapes::Point, shapes::Shape)> {
    let p1 = shapes::point(310.0, 101.0);
    let p2 = shapes::point(307.0, 105.0);
    let p3 = shapes::point(310.0, 109.0);
    let p4 = shapes::point(313.0, 105.0);

    vec![ (shapes::point(10.0, 10.0), shapes::rect(100.0, 200.0)),
          (shapes::point(150.0, 10.0), shapes::square(100.0)),
          (shapes::point(10.0, 250.0), shapes::ellipse(100.0, 200.0)),
          (shapes::point(150.0, 150.0), shapes::circle(100.0)),
          (shapes::point(300.0, 300.0), shapes::rt_triangle(100.0, 200.0)),
          (shapes::point(300.0, 100.0), shapes::polygon(vec![p1, p2, p3, p4]))]
}