
--------- Problem ---------

fn main() {
    for n in 1..=100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

--------- Solution ---------

fn main() {
    // Loop from 1 to 99 (100 is excluded)
    for n in 1..100 { 
        // Check if n equals 100 (never true)
        if n == 100 {
            panic!("NEVER LET THIS RUN")  // Never runs
        }
    }

    println!("Success!");  // Always runs
} 

