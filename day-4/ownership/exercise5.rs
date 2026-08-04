
--------- Problem ---------

// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

--------- Solution 1 ---------

fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");    
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
