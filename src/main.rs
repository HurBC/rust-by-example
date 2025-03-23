mod custom_types;
mod hello_world;
mod primitives;

// Constants are immutable values that are known at compile time, and are not allowed to change
// The are global values and they can be accessed from any part of the program
static LANGUAGE: &str = "Rust";
const ADULT_AGE: u32 = 18;

fn main() {
    println!("Hello World:");

    hello_world::hello_world();

    println!("formatted_print:");

    hello_world::formatted_print::formatted_print();

    println!("Primitives:");

    primitives::primitives();

    println!("Custom Types:");

    custom_types::custom_types();
}
