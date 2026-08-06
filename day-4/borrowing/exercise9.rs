--------- Problem ---------
// Ok: Borrow a mutable object as immutable
// This code has no errors!
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

--------- Solution ---------

// This code has no errors!
fn main() {
    let mut s: String = String::from("hello, ");

    // Borrow s immutably (read-only access)
    borrow_object(&s);     // &s = immutable reference
    
    // Modify s after the borrow is over
    s.push_str("world");      // Works because borrow_object ended

    println!("Success!");
}

fn borrow_object(s: &String) {}    // Takes immutable reference, does nothing

