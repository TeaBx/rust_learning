fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true        
    } else {
        false
    }
}

fn main() {
    
    for i in 0..=10 {
        println!(" Number {} is {}", i, if is_even(i) == true {"even"} else {"odd"});
    }

}
