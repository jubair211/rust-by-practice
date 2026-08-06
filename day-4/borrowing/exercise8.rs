--------- Problem ---------

fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

--------- Solution ---------

fn main() {
    // Create a mutable String
    let mut s: String = String::from("hello, ");

    // Pass a mutable reference to borrow_object
    borrow_object(&mut s);    // &mut s = mutable borrow

    println!("Success!");
}

fn borrow_object(s: &mut String) {}    // Function takes mutable reference but does nothing

