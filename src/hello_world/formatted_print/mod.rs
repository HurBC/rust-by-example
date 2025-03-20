mod debug;
mod display;
mod formating;

pub fn formatted_print() {
    // Normal
    println!("-{} days", 31);

    // Print with indexes
    println!("-{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Print with named args
    println!(
        "-{subject} {emotion} {receiver}",
        subject = "Hur",
        receiver = "Ellen",
        emotion = "Loves"
    );

    // Character format
    println!("-Base 10:               {}", 69420);
    println!("-Base 2 (binary):       {:b}", 69420);
    println!("-Base 8 (octal):        {:o}", 69420);
    println!("-Base 16 (hexadecimal): {:x}", 69420);

    // Add pad to output

    // Add blank space until the string length equals 5
    println!("-{number:>5}", number = 3);
    println!("-{number:0<5}", number = 3);
    println!("-{number:0>5}", number = 3);

    // Pad with named args
    println!("-{number:0>width$}", number = 3, width = 10);

    // This will be throw a panic
    // println!("-My name is {0}, {1} {0}", "Bond");

    // If a type doesn't implement the Display trait, this cannot be printed with {} in println!
    #[allow(dead_code)]
    struct Structure(i32);

    // This throw an panic!
    // println!("-Struct: {}", Structure(5));

    // SI
    let number: f64 = 1.0;
    let width: usize = 5;

    println!("-{number:>width$}");

    println!("-Debug:");
    debug::debug();

    println!("-Display:");
    display::display();

    println!("-Formatting:");
    formating::formatting();
}
