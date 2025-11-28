// Takes ownership and returns ownership
fn take_and_return(s: String) -> String {
   let mut r = s;
   r.push_str(" modifed");
   r
}

// Borrows immutably and returns new String
fn borrow_and_create(s: &String) -> String {
    let mut st = s.clone();
    st.make_ascii_uppercase();
    st
}

// Borrows mutably
fn borrow_and_modify(s: &mut String) {
   s.push_str(" modified");
}

fn main() {
    // Test all three functions
    let s1 = String::from("test");
    let s2 = take_and_return(s1);
    println!("{}", s2);
    
    let s3 = String::from("test");
    let s4 = borrow_and_create(&s3);
    println!("{} and {}", s3, s4);
    
    let mut s5 = String::from("test");
    borrow_and_modify(&mut s5);
    println!("{}", s5);
}