pub fn run() {
    //Print to console
    println!("hello from print.rs file!");

    //Basic formatting
    println!("Number: {}", 1);

    println!("{} plays {}", "Anurag", "Dota");

    //Positional Arguments
    println!("{0} is from {1} and {0} plays {2}", "Anurag", "Mid", "Dota");

    //Named Arguments
    println!(
        "{name} likes to code in {lang}",
        name = "Anurag",
        lang = "Rust"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
