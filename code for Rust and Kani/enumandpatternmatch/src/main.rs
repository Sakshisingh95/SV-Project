enum Shape {
    Circle(f64),         // Radius
    Rectangle(f64, f64), // Width, Height
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

fn main() {
    let circle = Shape::Circle(8.0);
    let rectangle = Shape::Rectangle(3.0, 9.0);
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}

