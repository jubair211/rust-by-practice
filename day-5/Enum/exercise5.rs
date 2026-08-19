--------- Problem ---------

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let __ = six {
        println!("{}", n);

        println!("Success!");
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        __ => None,
        __ => Some(i + 1),
    }
}

--------- Solution ---------

fn main() {
    // Create Some with value 5
    let five: Option<i32> = Some(5);
    
    // Call plus_one: Some(5) → Some(6)
    let six: Option<i32> = plus_one(five);
    
    // Call plus_one with None → None
    let none: Option<i32> = plus_one(None);

    // Pattern match: if six is Some, extract value
    if let Some(n) = six {  // n = 6
        println!("{}", n);  // Prints: 6

        println!("Success!");
    } else {
        // This runs if six is None (won't run)
        panic!("NEVER LET THIS RUN！");  
    }
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // If None, return None
        Some(i) => Some(i + 1), // If Some, add 1 and wrap in Some
    }
}

