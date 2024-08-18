use std::f64::consts::PI;

// Define an enum to represent different shapes
enum Shape {
    Circle(f64),         // Circle with radius
    Rectangle(f64, f64), // Rectangle with width and height
    Triangle(f64, f64),  // Triangle with base and height
}

// Implement methods for the Shape enum
impl Shape {
    // Method to calculate the area of the shape
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let triangle = Shape::Triangle(3.0, 7.0);

    // Print the area of each shape
    println!("Area of the circle: {:.2}", circle.area());
    println!("Area of the rectangle: {:.2}", rectangle.area());
    println!("Area of the triangle: {:.2}", triangle.area());
}

