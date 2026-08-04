
--------- Problem ---------

fn main() {
    let x = Box::new(5);
    
    let ...      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}

--------- Solution 1 ---------


fn main() {
    let x: Box<i32> = Box::new(5);
    
    let mut y: Box<i32> = Box::new(1);  // Transfer ownership to mutable y
    
    *y = 4;
    
    assert_eq!(*x, 5);  
    
    println!("Success!");
}
