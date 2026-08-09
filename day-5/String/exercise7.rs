--------- Problem ---------
// &str can be converted to String in two ways

// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}

--------- Solution 1 ---------

fn main() {
    let s: &str = "hello, world";
    greetings(s.to_string())  // &str -> String
}

fn greetings(s: String) {
    println!("{}", s)
}

--------- Solution 2 ---------

fn main() {
    let s: &str = "hello, world";
    greetings(String::from(s))  // &str -> String
}

fn greetings(s: String) {
    println!("{}", s)
}
