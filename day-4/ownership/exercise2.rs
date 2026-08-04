--------- Solution 1 ---------

// Don't modify code in main!
fn main() {
    // Create a String "Hello world" and store it in s1
    let s1 = String::from("Hello world");
    
    // Call take_ownership with s1, which MOVES ownership to the function
    // The function now returns the String, so s2 gets the value
    let s2 = take_ownership(s1);

    // Print the String stored in s2
    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {  // Add -> String to return a String
    // Print the received String
    println!("{}", s);
    
    // Return the String back to the caller
    // Ownership moves back to the caller (s2 in main)
    s  // No semicolon means "return this value"
}
--------- Solution 2 ---------

fn main() {
    // Create a new String "Hello world" on the heap
    // s1 OWNS this String
    let s1 = String::from("Hello world");
    
    // Call take_ownership and pass a REFERENCE to s1 (&s1)
    // We're BORROWING s1, not moving it
    // s1 still owns the String after this call
    let s2 = take_ownership(&s1);  // Pass a reference (borrow)
    
    // Print the String that s2 references
    // s2 is a reference, so we print what it points to
    println!("{}", s2);
}

// This function takes a REFERENCE to a String (borrows it)
// It returns a REFERENCE to a String
fn take_ownership(s: &String) -> &String {  // Work with references
    // Print the String that s points to
    // s is a reference, so we don't own the String
    println!("{}", s);
    
    // Return the reference back to the caller
    // We're just passing the reference along
    s  // Return the reference (no semicolon means "return this")
}
