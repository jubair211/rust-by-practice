--------- Problem ---------

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = __;
    
    p.push_str("world");

    println!("Success!");
}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello, ");

    // Create a mutable reference p that borrows s
    let p: &mut String = &mut s;  // p can modify s
    
    // Use p to modify the String
    p.push_str("world");  // s becomes "hello, world"

    println!("Success!");  // Prints success message
}

