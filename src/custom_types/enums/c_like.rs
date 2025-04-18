enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum LinuxOs {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub(super) fn c_like_enums() {
    println!("--Zero is {}", Number::Zero as i32);
    println!("--One is {}", Number::One as i32);

    println!("--Red is 0x{:06x}", Color::Red as i32);
    println!("--Green is 0x{:06x}", Color::Green as i32);
    println!("--Blue is 0x{:06x}", Color::Blue as i32);
}
