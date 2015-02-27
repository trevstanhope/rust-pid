// main of pid.rs
extern crate pid;

#[test]
fn it_works() {
}

// main
fn main() {
    let s = "something happened";
    pid::pretty_print(s); // uses a function from lib.rs
    
    let x: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    pid::mean(&x);
    pid::median(&x);
}
