--------- Problem ---------

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello, ");     // s is mutable

    borrow_object(&s);     // Pass immutable reference (no modification)

    println!("Success!");
}

fn borrow_object(s: &String) {}    // Takes immutable reference

