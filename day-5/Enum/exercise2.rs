--------- Problem ---------

// Fill in the blank
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move{__}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write(__); // Instantiating with "hello, world!"

    println!("Success!");
}

--------- Solution ---------

// Define an enum with different variant types
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },                       // Named fields (struct-like)
    Write(String),                                     // Tuple variant (one String)
    ChangeColor(i32, i32, i32),             // Tuple variant (three i32s)
}

fn main() {
    // Create a Move variant with named fields
    let msg1 = Message::Move { x: 1, y: 2 };                    // Struct-like syntax
    
    // Create a Write variant with a String
    let msg2 = Message::Write(String::from("hello, world!"));            // Tuple-like syntax

    println!("Success!");
}

