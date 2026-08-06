--------- Problem ---------

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;   // First mutable borrow
    let r2: &mut String = &mut s;   // Second mutable borrow (r1 invalid)

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    println!("{}, {}", r1, r2);     // ERROR! r1 and r2 can't coexist
}

