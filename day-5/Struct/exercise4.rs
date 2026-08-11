--------- Problem ---------
// You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.

// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18? 
    p.age = 30;

    // Fill the blank
    __ = String::from("sunfei");

    println!("Success!");
}

--------- Solution ---------

// Define a struct Person with name and age fields
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age: u8 = 18;  // Age variable
    
    // Create mutable Person instance using field init shorthand
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,  // age: age (shorthand)
    };

    // Update age to 30 (requires p to be mutable)
    p.age = 30;     // Change age field

    // Update name to "sunfei" (requires p to be mutable)
    p.name = String::from("sunfei");    // Change name field

    println!("Success!");
}

