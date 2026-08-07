--------- Problem ---------

// Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}

--------- Solution ---------

fn main() {
    // Create a mutable String
    let mut s: String = String::from("hello");
    
    // Push a single character
    s.push(',');          // s = "hello,"
    
    // Push a string slice
    s.push_str(" world");   // s = "hello, world"
    
    // Use += operator to concatenate
    s += "!";             // s = "hello, world!"

    println!("{}", s);    // Output: hello, world!
}
