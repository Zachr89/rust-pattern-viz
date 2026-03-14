//! Example: Struct Destructuring
//! 
//! Demonstrates pattern matching with struct field extraction.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    alpha: Option<u8>,
}

fn describe_point(point: Point) -> String {
    // Pattern 1: Basic struct destructuring
    let Point { x, y } = point;
    format!("Point at ({}, {})", x, y)
}

fn is_origin(point: &Point) -> bool {
    // Pattern 2: Destructuring with matching specific values
    match point {
        Point { x: 0, y: 0 } => true,
        _ => false,
    }
}

fn calculate_area(rect: &Rectangle) -> i32 {
    // Pattern 3: Nested struct destructuring
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    
    (x2 - x1).abs() * (y2 - y1).abs()
}

fn describe_color(color: Color) -> String {
    // Pattern 4: Destructuring with Option field
    match color {
        Color { r, g, b, alpha: Some(a) } => {
            format!("RGBA({}, {}, {}, {})", r, g, b, a)
        }
        Color { r, g, b, alpha: None } => {
            format!("RGB({}, {}, {})", r, g, b)
        }
    }
}

fn is_on_axis(point: &Point) -> bool {
    // Pattern 5: Partial destructuring with ..
    match point {
        Point { x: 0, .. } => true,  // On Y-axis
        Point { y: 0, .. } => true,  // On X-axis
        _ => false,
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}", describe_point(p));
    
    let origin = Point { x: 0, y: 0 };
    println!("Is origin: {}", is_origin(&origin));
    
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 5, y: 0 },
    };
    println!("Area: {}", calculate_area(&rect));
    
    let color = Color { r: 255, g: 0, b: 0, alpha: Some(128) };
    println!("{}", describe_color(color));
}
