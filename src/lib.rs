extern crate time;

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
