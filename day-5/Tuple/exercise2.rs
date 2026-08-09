--------- Problem ---------

// Make it work
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}
--------- Solution ---------

fn main() {
    let t: (&str, &str, &str) = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}
