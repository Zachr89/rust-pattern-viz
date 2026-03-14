export const EXAMPLE_CODE = {
  errorHandling: `// Example: Error Handling with Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn process_data(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let num: i32 = input.parse()?;
    let result = divide(num, 2)?;
    Ok(result * 3)
}`,

  iterators: `// Example: Iterator Chains
fn process_numbers(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .filter(|&&x| x > 0)
        .map(|x| x * 2)
        .collect()
}

fn sum_even_squares(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .fold(0, |acc, x| acc + x)
}`,

  lifetimes: `// Example: Lifetime Annotations
struct Parser<'a> {
    content: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Parser { content, position: 0 }
    }
    
    fn peek(&self) -> Option<&'a str> {
        self.content.get(self.position..self.position + 1)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}`,

  traits: `// Example: Trait Implementation
trait Drawable {
    fn draw(&self) -> String;
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle {}x{}", self.width, self.height)
    }
}`,
};
