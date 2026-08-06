--------- Problem ---------
// We can only use str by boxing it, & can be used to convert Box<str> to &str

// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

--------- Solution 1 ---------

fn main() {
    // Create a Box<str> (string on the heap)
    // .into() converts &str to Box<str>
    let s: Box<str> = "hello, world".into();
    
    // Pass a reference to s to greetings
    // &s is &Box<str>, but greetings expects &str
    greetings(&s)  
}

fn greetings(s: &str) {
    println!("{}", s)
}

--------- Solution 2 ---------

fn main() {
    // &str is an immutable reference to a fixed string in binary
    let s: &str = "hello, world";
    
    // Pass s directly to greetings
    // s is already &str, so no conversion needed!
    greetings(s)  // ✅ Works perfectly
}

fn greetings(s: &str) {
    println!("{}", s)
}
