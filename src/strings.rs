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

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
