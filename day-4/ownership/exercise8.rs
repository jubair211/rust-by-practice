
--------- Problem ---------


fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}

--------- Solution 1 ---------


fn main() {
    // Create a tuple with two Strings
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Move ownership of first element to _s
    // t.0 is now invalid (moved)
    let _s = t.0;
    println!("{:?}", t.1);      //  t.1 is still valid (not moved)
}

