--------- Problem ---------

// Fill in the blank and fix the errors
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: __ = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{}", msg);
}

--------- Solution ---------

// Derive Debug trait to enable printing with {:?}
#[derive(Debug)] 
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct-like variant
    Write(String),              // Tuple variant
    ChangeColor(i32, i32, i32), // Tuple variant
}

fn main() {
    // Create an array of 3 Message variants
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0)
    ];

    // Iterate over each message and call show_message
    for msg in msgs {
        show_message(msg)  // msg moves into function
    }
} 

fn show_message(msg: Message) {
    // Print message using Debug formatting
    println!("{:?}", msg);  // Works because of #[derive(Debug)]
}

