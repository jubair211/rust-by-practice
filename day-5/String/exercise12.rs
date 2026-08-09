--------- Problem ---------

fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".__ {
        println!("{}", c)
    }
}

--------- Solution ---------

fn main() {
    // Iterate over each character in the Chinese string
    // .chars() returns an iterator over Unicode characters
    for c in "你好，世界".chars() {
        println!("{}", c)  // Print each character on a new line
    }
}
