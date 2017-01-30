
mod shapes;

fn main() {
    println!("Rectangle {:?}", shapes::rect(100.0, 200.0));
    println!("Square {:?}", shapes::square(100.0));
    println!("Ellipse {:?}", shapes::ellipse(100.0, 200.0));
    println!("Circle {:?}", shapes::circle(100.0));
    println!("Right Traingle {:?}", shapes::rt_triangle(100.0, 200.0));
    println!("Polygon {:?}", shapes::polygon(vec![100.0, 200.0, 300.0]));
}
