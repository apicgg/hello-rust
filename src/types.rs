pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicit type
    let z: i64 = 27463746376436;

    // Find max size
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i64 is {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Boolean from expresion
    let is_greater: bool = 10 > 20;

    //Char
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
