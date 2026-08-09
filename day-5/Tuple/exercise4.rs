--------- Problem ---------

fn main() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let __ = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Create a tuple with 3 values: i32, f64, &str
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // Destructure the tuple into 3 variables
    let (x, z, y) = tup;      // x=1, z=6.4, y="hello"

    // Verify values
    assert_eq!(x, 1);      //  i32 value
    assert_eq!(y, "hello");   // &str value
    assert_eq!(z, 6.4);    // f64 value

    println!("Success!");
}
