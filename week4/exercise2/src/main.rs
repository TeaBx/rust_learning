fn main() {
    let mut s = String::from("hello");
    make_uppercase(&mut s);
    println!("{}", s); 
}

fn make_uppercase(s: &mut String) {
    s.make_ascii_uppercase();
}


