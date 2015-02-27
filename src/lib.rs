#![feature(test)]
extern crate time;
extern crate test;

#[test]
fn it_works() {
}

// pretty print
pub fn pretty_print(e : &str) {
    let t = time::now();
    let s = time::strftime("%Y-%m-%d %H:%M:%S", &t);
    if s.is_ok() {
    println!("{0}\t{1}", s.unwrap(), e);
    }
    else {
        println!("####-##-## ##:##:##\t{}", e);
    }
}

// mean of array
pub fn mean(a : &[f32]) -> f32 {
    let mut sum = 0.0;
    let mut n = 0.0;
    for p in a.iter() {
        sum = sum + *p; // need to (*) dereference pointers
        n = n + 1.0;
    }
    let f = sum / n;
    return f;
}

// median of array
pub fn median(a : &[f32]) -> f32 {
    let l = a.len();
    let f = a[l];
    return f;
}
