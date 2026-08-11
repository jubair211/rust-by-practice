--------- Problem ---------

// Fill the blanks to make the code work
#[__]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!(__, rect1); // Print debug info to stdout
}
--------- Solution ---------

// Derive Debug trait to enable printing Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        // dbg! prints the expression and returns the value
        // Prints: [src/main.rs:12] 30 * scale = 60
        width: dbg!(30 * scale), // Assigns 60 to width
        height: 50,
    };

    // dbg! prints the value and returns it (to stderr)
    // Prints: [src/main.rs:16] &rect1 = Rectangle { width: 60, height: 50 }
    dbg!(&rect1);

    // println! prints to stdout (standard output)
    println!("{:?}", rect1);  // Outputs: Rectangle { width: 60, height: 50 }
}
