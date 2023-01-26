pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4297423;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    let is_greater = 10 < 5;

    // char
    let a1 = '🍕';
    let face = '\u{1f601}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
