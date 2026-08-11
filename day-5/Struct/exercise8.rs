--------- Problem ---------


// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}, {:?}",f.name, f.data, f);
}

--------- Solution ---------

// Derive Debug trait to enable printing with {:?}
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    // Create a File instance
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    // Move ownership of name field to _name
    // f.name is moved, so f.name is no longer valid
    let _name: String = f.name;  // f.name MOVES to _name

    // f.data is still valid (not moved)
    // f.name would be invalid (moved)
    println!("{}, {}", _name, f.data);  // Works: _name + f.data
}

