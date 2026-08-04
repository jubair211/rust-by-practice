
--------- Problem ---------


// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");
    
    let s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

--------- Solution 1 ---------

fn main() {
    let s: String = String::from("Hello ");

    let mut s1 = s;    // Make s1 mutable when ownership transfers

    s1.push_str("World!");
    println!("Success!");
}
