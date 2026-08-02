--------- Solution 1 ---------

fn main() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: String = x.clone();      // create a deep copy
    println!("{}, {}",x, y);          // Both work!
} 

--------- Solution 2 ---------

fn main() {
    let x: String = String::from("Hello world");
    let y: String = &x;              // y borrows x (doesn't take ownership)
    println!("{}, {}", x, y);         // Both work!
}

--------- Solution 2 ---------

fn main() {
    let x: String = String::from("Hello world");
    println!("{}", x);       // Use x before moving
    let y: String = x;           // Now move it
    println!("{}", y);   // Use y
}
