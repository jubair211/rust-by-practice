--------- Problem ---------

// Fix the error
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}
--------- Solution ---------

fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);  // in Tuple 12 elements can be ptinted
    println!("too long tuple: {:?}", too_long_tuple);
}
