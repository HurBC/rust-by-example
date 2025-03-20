mod hello_world;
mod primitives;

fn main() {
    println!("Hello World:");

    hello_world::hello_world();

    println!("formatted_print:");

    hello_world::formatted_print::formatted_print();
}
