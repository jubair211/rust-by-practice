
--------- Problem ---------

// Fill in the blanks to make the last println! work !
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n __ 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        __;
    }

    println!("n reached {}, so loop is over",n);
}

--------- Solution ---------

fn main() {
    // A counter variable
    let mut n: i32 = 1;  // Must be mutable to increment

    // Loop while the condition is true
    while n < 10 {  // Runs while n is 1 to 9
        if n % 15 == 0 {         // Divisible by 15 (3 and 5)
            println!("fizzbuzz");
        } else if n % 3 == 0 {   // Divisible by 3
            println!("fizz");
        } else if n % 5 == 0 {   // Divisible by 5
            println!("buzz");
        } else {                 // Not divisible by 3 or 5
            println!("{}", n);
        }

        n += 1;  // Increment counter (n = n + 1)
    }

    // Loop ends when n = 10
    println!("n reached {}, so loop is over", n);
}

