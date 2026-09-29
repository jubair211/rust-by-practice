
--------- Problem ---------

// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
    }
    
    println!("{:?}", numbers);
} 

--------- Solution ---------

fn main() {
    // Array of 2 Strings (String does NOT implement Copy)
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
    
    // Iterate by REFERENCE (&names) so names is not moved
    for name in &names {
        println!("{}", name);  // name is &String
    }

    // names still valid (we borrowed, not moved)
    println!("{:?}", names);

    // Array of 3 i32 (i32 implements Copy)
    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy, so there is no move here
    for n in numbers {
        println!("{}", n)  // n is i32 (copied)
    }
    
    // numbers still valid (i32 is Copy, so it was copied)
    println!("{:?}", numbers);
}

