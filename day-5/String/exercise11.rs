--------- Problem ---------

fn main() {
    let s1 = String::from("hi,中国");
    let h = s1[0]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}

--------- Solution 1 ---------

fn main() {
    // Create a String with ASCII and Chinese characters
    let s1: String = String::from("hi,中国");
    //     h  i  ,  中  国
    //     |  |  |  |   |
    //    [0][1][2][3] [6]  (byte indices)
    
    // Slice from byte 0 to 1 (exclusive) -> "h"
    let h: &str = &s1[0..1]; 
    assert_eq!(h, "h");    //  "h" matches
    
    // Slice from byte 3 to 6 (exclusive) -> "中" (3 bytes in UTF-8)
    let h1 = &s1[3..6];  
    assert_eq!(h1, "中");    //  "中" matches

    println!("Success!");
}
