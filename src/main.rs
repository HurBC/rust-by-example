mod custom_types;
mod hello_world;
mod primitives;

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
