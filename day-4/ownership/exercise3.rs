--------- Solution 1 ---------

fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    // Convert String to Vec
    let _s = s.as_bytes();     //  as_bytes() BORROWS s (doesn't take ownership)
    s                  //  s is still valid because we only borrowed it
}

