fn main() {
    
    let records = [("Tea", 30u8, 7.9f32), ("Ana", 25u8, 8.1f32), ("Nina", 31u8, 6.5f32)];
    
    println!("{:<10} {:<9} {:<10}", "Name", "Age", "Grade");
    println!("{:-<28}","");

    let mut sum: f32 = 0.0;
    for (name, age, grade) in records {
        println!("{:<12}  {:>3}  {:>7.2}", name, age, grade);
        sum += grade;
    }

    let avg = sum / records.len() as f32;

    println!("\nAverage class grade: {:.2}", avg);

}
