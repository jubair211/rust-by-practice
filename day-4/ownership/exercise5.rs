
--------- Problem ---------

// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

--------- Solution 1 ---------

fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");    // Use &str (string literal, is Copy)
    let y: (i32, i32, (), &str) = x;      // x is automatically copied
    println!("{:?}, {:?}", x, y);
}

--------- Solution 2 ---------

fn main() {
    let x: (i32, i32, (), String) = (1, 2, (), "hello".to_string());
    let y = &x;    // Borrow x instead of copying
    println!("{:?}, {:?}", x, y);
}
