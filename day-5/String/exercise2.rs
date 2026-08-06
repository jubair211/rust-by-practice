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

--------- Solution ---------

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
