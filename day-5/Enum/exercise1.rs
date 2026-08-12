--------- Problem ---------


// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One, Number1::One);
    assert_eq!(Number1::One, Number2::One);

    println!("Success!");
}

--------- Solution ---------

// Enum without explicit values (starts from 0 by default)
enum Number {
    Zero,  // 0
    One,   // 1
    Two,   // 2
}

// Enum with first value explicitly set to 0
// Others auto-increment: One=1, Two=2
enum Number1 {
    Zero = 0,    // 0
    One,         // 1
    Two,          // 2
}

// C-like enum with all values explicitly set
enum Number2 {
    Zero = 0,      // 0
    One = 1,      // 1
    Two = 2,     // 2
}

fn main() {
    // Convert enum variants to integers using `as`
    // All enums have One = 1
    assert_eq!(Number::One as u8, Number1::One as u8);    // 1 == 1 
    assert_eq!(Number1::One as u8, Number2::One as u8);    // 1 == 1 

    println!("Success!");
}

