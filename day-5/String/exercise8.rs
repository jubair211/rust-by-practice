--------- Problem ---------

// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}

--------- Solution 1 ---------

fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s.as_str();  // &String -> &str

    println!("Success!");
}
