--------- Problem ---------

fn main() {
    // Fill the blank
    let list: [i32; 100] = __ ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Fill the blank
    let list: [i32; 100] = [1; 100] ;  // [1, 1, 1, ..., 1]

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
