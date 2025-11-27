
fn collatz_sequence(mut n: u32) -> u32 {

    let mut steps = 0;
    
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        steps += 1; 
    }
    return steps
}

fn main() {
    
    let n = 6;
    let steps = collatz_sequence(n);

    println!("\nNumber of steps for {} is {}", n, steps);

}
