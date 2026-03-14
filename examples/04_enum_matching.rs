//! Example: Enum Pattern Matching
//! 
//! Demonstrates comprehensive enum variant matching patterns.

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

#[derive(Debug)]
enum PaymentMethod {
    Cash,
    CreditCard { number: String, cvv: u16 },
    Crypto { wallet: String, amount: f64 },
}

fn process_message(msg: Message) {
    // Pattern 1: Exhaustive enum matching
    match msg {
        Message::Quit => {
            println!("Quit command received");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

fn handle_event(event: WebEvent) -> String {
    // Pattern 2: Match with return values
    match event {
        WebEvent::PageLoad => "Page loaded".to_string(),
        WebEvent::PageUnload => "Page unloaded".to_string(),
        WebEvent::KeyPress(c) => format!("Key pressed: {}", c),
        WebEvent::Paste(s) => format!("Pasted: {}", s),
        WebEvent::Click { x, y } => format!("Clicked at ({}, {})", x, y),
    }
}

fn validate_payment(method: PaymentMethod) -> Result<(), String> {
    // Pattern 3: Match with guards (if conditions)
    match method {
        PaymentMethod::Cash => Ok(()),
        PaymentMethod::CreditCard { number, cvv } if cvv >= 100 && cvv <= 999 => {
            if number.len() == 16 {
                Ok(())
            } else {
                Err("Invalid card number length".to_string())
            }
        }
        PaymentMethod::CreditCard { .. } => {
            Err("Invalid CVV".to_string())
        }
        PaymentMethod::Crypto { amount, .. } if amount > 0.0 => Ok(()),
        PaymentMethod::Crypto { .. } => {
            Err("Invalid crypto amount".to_string())
        }
    }
}

fn is_interactive(event: &WebEvent) -> bool {
    // Pattern 4: Match with references and wildcards
    matches!(event, WebEvent::KeyPress(_) | WebEvent::Click { .. })
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);
    
    let event = WebEvent::Click { x: 100, y: 200 };
    println!("{}", handle_event(event));
    
    let payment = PaymentMethod::CreditCard {
        number: "1234567890123456".to_string(),
        cvv: 123,
    };
    match validate_payment(payment) {
        Ok(()) => println!("Payment valid"),
        Err(e) => println!("Payment error: {}", e),
    }
}
