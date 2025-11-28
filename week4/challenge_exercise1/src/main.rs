// fn process_strings() -> String {
//     let mut s1 = String::from("hello");
//     let s2 = String::from(" world");
    
//     let r1 = &mut s1;
//     r1.push_str(&s2);
    
//     let r2 = &s1;
//     let r3 = &mut s1;    // there can;t be both mutable and immutable references of the same value in the same lifetime of a reference 
    
//     r3.push_str("!");
    
//     println!("{}", r2);
    
//     r3              // function expects String as a return value, but this is &mut String. it points to local variable which will be lost after 
//                     // function call and r3 will be invalid
// }

fn process_strings() -> String {
    let mut s1 = String::from("hello");
    let s2 = String::from(" world");
         
    let r1 = &mut s1;
    r1.push_str(&s2);
    r1.push_str("!");
    println!("{r1}");
    
    s1           
}




fn main() {
    let result = process_strings();
    println!("{}", result);
}