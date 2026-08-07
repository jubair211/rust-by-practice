--------- Problem ---------

// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.__("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

--------- Solution ---------

fn main() {
    // Create a mutable String
    let mut s: String = String::from("I like dogs");
    
    // Allocate new memory and store the modified string there
    // replace() creates a NEW String, doesn't modify original
    let s1 = s.replace("dogs", "cats");  // s1 = "I like cats", s unchanged

    // Check if s1 equals expected
    assert_eq!(s1, "I like cats");    // Matches

    println!("Success!");
}
