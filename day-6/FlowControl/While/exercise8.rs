
--------- Problem ---------

// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           __;
       }
       
       __
    }

    assert_eq!(n, 66);

    println!("Success!");
}

--------- Solution ---------

fn main() {
    let mut n = 0;
    for i in 0..=100 {          // Loop from 0 to 100 (inclusive)
       if n != 66 {             // If n is NOT 66
           n += 1;              // Increment n
           continue;            // Skip rest, go to next iteration
       }
       
       break;                   // If n IS 66, exit loop
    }

    assert_eq!(n, 66);          // n should be 66 

    println!("Success!");
}

