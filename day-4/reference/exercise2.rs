
--------- Problem ---------

fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, y);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    let x: i32 = 5;
    
    // Create a reference y that points to x
    let y: &i32 = &x;

    // *y dereferences y to get the actual value (5)
    assert_eq!(5, *y);      //  *y gets the value 5 from the reference

    println!("Success!");
}

