fn main() {
   
    let tuple: (i8, f32, u8) = (-123, 45.6, 78);
    
    println!("tuple: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let (a, b, c) = tuple;

    println!("destructured tuple: {}, {}, {}", a, b, c);

    let sum = a as f32 + b + c as f32;
    let average = sum/3.0;
    let product = a as f32 * b * c as f32;

    println!("sum: {} \naverage: {}\nproduct: {}", sum, average, product);

}
