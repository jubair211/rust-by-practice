--------- Problem ---------
// We can't use str type in normal ways, but we can use &str.

// Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Create a string slice (string literal) pointing to "hello, world"
    // &str is an immutable reference to a fixed string in binary
    let s: &str = "hello, world";  // s is a string literal

    println!("Success!");
}

