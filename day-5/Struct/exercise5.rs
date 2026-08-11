--------- Problem ---------

// Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        __
    }
}
--------- Solution ---------

// Define a struct Person with name and age fields
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Success!");
} 

// Function that creates and returns a Person instance
fn build_person(name: String, age: u8) -> Person {
    Person {
        age,    // Field init shorthand for age: age
        name,   // Field init shorthand for name: name
        // Order doesn't matter! Fields can be in any order
    }
}

