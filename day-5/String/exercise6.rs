--------- Problem ---------

// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

--------- Solution 1 ---------

fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    // s1 + &s2 works because:
    // s1 is String, &s2 is &str (String -> &str via as_str())
    // s1 is MOVED here, s2 is BORROWED
    let s3: String = s1 + s2.as_str(); // String -> &str
    assert_eq!(s3, "hello,world!");  // Concatenated correctly
    println!("{}", s3);              // Output: hello,world!
}

--------- Solution 1 ---------

fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; // &String -> &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}
