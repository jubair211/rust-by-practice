
--------- Problem ---------

fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}

--------- Solution ---------

fn main() {
    // Create an array of 4 integers
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    // a.iter() = iterator over references to elements
    // .enumerate() = adds index to each element
    for (i, v) in a.iter().enumerate() {
        // i = index (0, 1, 2, 3)
        // v = value (&i32)
        println!("The {}th element is {}", i + 1, v);  // i+1 for 1-based counting
    }
}

