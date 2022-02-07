pub fn run() {
    // Primitive str = Immutable fixed length
    let hello = "Hello";
    println!("{}", hello);

    // String = Growable, heap allocated data structure
    let mut hello1 = String::from("Hello ");

    // Get length
    println!("Length: {}", hello1.len());

    hello1.push('W');
    hello1.push_str("orld!");

    println!("{}", hello1);
    println!("Length: {}", hello1.len());

    // Capacity in bytes
    println!("Capacity: {}", hello1.capacity());
}
