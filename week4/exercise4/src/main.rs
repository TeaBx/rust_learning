fn main() {
    //let s1 = String::from("hello");
    // let s2 = s1;

    //println!("{}", s1); // will this compile?  - no, because ownership of string has been moved to s2, so s1 is invalid now. 

    {   
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
    }

    {
        let s1 = String::from("hello");
        let s2 = &s1;
        println!("s1 = {s1}, s2 = {s2}");
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("s2 = {s2}");
    }

}
