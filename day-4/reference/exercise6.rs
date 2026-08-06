--------- Problem ---------

fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let __ r2 = c;

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

--------- Solution ---------

fn main() {
    // Create a character variable
    let c: char = '中';

    // Create a reference to c (explicit way)
    let r1: &char = &c;
    
    // Create a reference to c using the 'ref' keyword (pattern syntax)
    let ref r2 = c;  // Same as: let r2 = &c;

    // Dereference both to compare actual values
    assert_eq!(*r1, *r2);  // Both point to '中'
    
    // Check that both references point to the SAME memory address
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)  // {:p} prints the memory address
}
