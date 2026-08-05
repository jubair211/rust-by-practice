
--------- Problem ---------



fn main() {
   let x = 5;
   // Fill the blank
   let p = __;

   println!("the memory address of x is {:p}", p);     // One possible output: 0x16fa3ac84
}

--------- Solution ---------


fn main() {
    let x: i32 = 5;
    
    // Create a reference p that points to x
    // &x means "borrow x" - p stores the memory address of x
    let p: &i32 = &x;  // p is a reference to x

    // {:p} is a format specifier for printing memory addresses (pointers)
    // p holds the memory address where x is stored
    println!("the memory address of x is {:p}", p); 
} 
