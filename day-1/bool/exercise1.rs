fn main() {
    let _f: bool = false;  // _f    underscore _ tells Rust: "I'm not going to use this variable" - no warning will be shown
    let t: bool = true;
    if t {
        println!("Success!");
    }
}
