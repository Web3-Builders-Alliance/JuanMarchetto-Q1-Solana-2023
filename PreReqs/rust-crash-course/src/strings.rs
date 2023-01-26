pub fn run() {
    let mut hello = String::from("Hello");

    //Get length
    println!("length: {}", hello.len());

    hello.push(' ');
    hello.push_str("World!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("is empty: {}", hello.is_empty());

    println!("Contains 'World' {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "WBA"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("{}", s);
}