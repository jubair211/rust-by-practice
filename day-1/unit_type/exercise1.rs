fn main() {
    let _v: () = (); // _f    underscore _ tells Rust: "I'm not going to use this variable" - no warning will be shown
    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}
// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
