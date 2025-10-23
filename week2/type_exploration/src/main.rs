fn main() {
    
    //integers
    let int8: i8 = -111;
    let unit8: u8 = 111;
    let int16: i16 = -5555;
    let uint16: u16 = 5555;
    let int32: i32 = -2_000_000_000;
    let uint32: u32 = 4_000_000_000;
    let int64: i64 = -5_000_000_000;
    let uint64: u64 = 10_000_000_000;
    let int128: i128 = -100_000_000_000_000;
    let uint128: u128 = 100_000_000_000_000;

    let int_size: isize = 64;
    let uint_size: usize = 64;

    //floating-point

    let float32: f32 = 2025.2310;
    let float64 = 23.10;

    //boolean
    let t: bool = true;
    let f = false;

    let a = t && f;
    let o = t || f;
    let n = !f;

    let c1: char = 't';
    let c2 = 'B';
    let c3 = '😻';

    println!("\n===== BOOLEAN =====");
    println!("t: {}, f: {}", t, f);
    println!("t && f = {}", a);
    println!("t || f = {}", o);
    println!("!f     = {}", n);

    println!("\n===== CHAR =====");
    println!("c1: {}", c1);
    println!("c2: {}", c2);
    println!("c3: {}", c3);


}
