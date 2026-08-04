
--------- Problem ---------

// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

--------- Solution 1 ---------

fn main() {
    let s: String = String::from("Hello World");

    print_str(s.clone());    //  Clone s before passing (s stays valid)

    println!("{}", s);      //  s still owns the String
}

fn print_str(s: String)  {
    println!("{}",s)
}
