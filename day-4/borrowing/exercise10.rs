--------- Problem ---------

// Comment one line to make it work
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}",r1);
}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;     // First mutable borrow
    r1.push_str("world");             // Modify through r1
    let r2: &mut String = &mut s;     // Second mutable borrow (invalidates r1)
    r2.push_str("!");                 // Modify through r2
    
    // println!("{}", r1);            //  COMMENT THIS LINE - r1 is no longer valid!
}

