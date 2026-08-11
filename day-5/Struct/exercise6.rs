--------- Problem ---------

// Fill the blank to make the code work
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        __
    }
}

--------- Solution ---------

// Define a struct User with 4 fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Create a User instance u1
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    // Pass u1 to set_email, returns a new User with updated email
    // u1 is MOVED here (ownership transferred to function)
    let u2: User = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),  // New email
        ..u  // Struct update syntax: use remaining fields from u
    }
    // u is moved into the new struct
}

