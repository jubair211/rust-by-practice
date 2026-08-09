--------- Problem ---------

fn main() {
    // Fill the blank with proper array type
    let arr: __ = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Create an array of i32 with 5 elements
    // [T; N] = type: T with exactly N elements
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Check if array length is 5
    assert!(arr.len() == 5);     // 5 == 5, passes

    println!("Success!");
} 
