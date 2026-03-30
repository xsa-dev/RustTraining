// Challenge: 
// Define an enum Shape with 
// variants Circle(f64) (radius), Rectangle(f64, f64) (width, height), and Triangle(f64, f64) (base, height). 
// Implement a method fn area(&self) -> f64 using match. Create one of each and print the area.
use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(w,h) => 0.5 * w * h,
        } 
    }
}

fn main() {
    let shapes = [
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 5.0),
        Shape::Triangle(22.0, 33.0)
    ];

    for shape in &shapes {
        println!("Aread {:.2}", shape.area());
    }
}
