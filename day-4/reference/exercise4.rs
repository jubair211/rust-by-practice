--------- Problem ---------

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

--------- Solution ---------

fn main() {
    let mut s: String = String::from("hello, ");  

    push_str(&mut s);  // Pass mutable reference (allows modification)

    println!("Success!"); 
}

fn push_str(s: &mut String) {  // Function expects mutable reference
    s.push_str("world");       // Modifies the original string
}
