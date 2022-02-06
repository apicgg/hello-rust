pub fn run() {
    let name = "Anurag";
    let mut age = 25;
    println!("My namne is {} and I am {} years old", name, age);
    age = 26;

    println!("My namne is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multple vars
    let (my_name, my_age) = ("Anurag", 25);
    println!("My name is {} and I am {} years old", my_name, my_age);
}
