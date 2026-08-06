--------- Problem ---------

// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello");

    // Create two immutable references to s
    let r1: &String = &s;    // r1 borrows s
    let r2: &String = &s;    // r2 also borrows s (multiple immutable refs allowed)

    // Print both references (they point to "hello")
    println!("{}, {}", r1, r2);

    println!("Success!");
}

