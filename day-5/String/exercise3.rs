--------- Problem ---------

// Fill the blank
fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Create an empty mutable String
    let mut s: String = String::from("");
    
    // Append string slice to s
    s.push_str("hello, world");    // s = "hello, world"
    
    // Append single character to s
    s.push('!');                 // s = "hello, world!"

    // Check if s equals the expected string
    assert_eq!(s, "hello, world!");    // Both equal

    println!("Success!");
}
