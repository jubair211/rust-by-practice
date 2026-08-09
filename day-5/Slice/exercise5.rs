--------- Problem ---------

fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..2];

    assert!(slice == "你");

    println!("Success!");
}

--------- Solution ---------


fn main() {
    let s: &str = "你好，世界";
    
    let slice: &str = &s[0..3];  // '你' take 3 bytes or each uni code takes 3 bytes

    assert!(slice == "你");

    println!("Success!");
}

