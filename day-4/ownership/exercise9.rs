
--------- Problem ---------


fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t);    // -> "hello", "world", ("hello", "world")
}


--------- Solution 1 ---------


fn main() {
    // Create a tuple with two Strings
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Clone the entire tuple t to create s1 and s2
    // s1 gets the first element, s2 gets the second element
    let (s1, s2) = t.clone();        // t remains valid because we cloned it

    // Print all three: s1, s2, and the original tuple t
    println!("{:?}, {:?}, {:?}", s1, s2, t);      // -> "hello", "world", ("hello", "world")
}
