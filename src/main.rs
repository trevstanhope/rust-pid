// main of pid.rs
extern crate pid;

#[test]
fn it_works() {
}

// main
fn main() {
    let s = "something happened";
    pid::pretty_print(s); // uses a function from lib.rs
}
