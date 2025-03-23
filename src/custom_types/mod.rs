mod structs;
mod enums;
mod constants;

pub fn custom_types() {
    println!("-Structs:");
    structs::structs();

    println!("-Enums:");
    enums::enums();

    println!("-Constants:");
    constants::constants();
}
