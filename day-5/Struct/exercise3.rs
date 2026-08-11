--------- Problem ---------

// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(__, __, __);
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
 }

--------- Solution ---------

// Define a tuple struct Color with 3 i32 fields
struct Color(i32, i32, i32);

// Define a tuple struct Point with 3 i32 fields
struct Point(i32, i32, i32);

fn main() {
    // Create a Point instance with values (0, 127, 255)
    let v: Point = Point(0, 127, 255);
    
    // Pass Point to check_color function
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Point) {
    // Destructure Point: bind first to x, ignore second with _, bind third to z
    let Point(x, _, z) = p;      // x=0, _=127, z=255
    
    // Verify values
    assert_eq!(x, 0);          // First field is 0
    assert_eq!(p.1, 127);       // Second field is 127 (accessed directly)
    assert_eq!(z, 255);         // Third field is 255
}
