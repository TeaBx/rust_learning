fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a > b && a > c {
        a
    } else if b > a && b > c {
        b
    } else {
        c
    }
}

fn main() { 
    let a = 54;
    let b = 72;
    let c = 65;
    
    let max = max_of_three(a, b, c);

    println!("Max number is {}", max);
}
