// This struct cannot be printed because not implementing Debug or Display trait
#[allow(dead_code)]
struct UnPrintable(f32);

// This struct can be printed because implement the Debug Trait
#[derive(Debug)]
#[allow(dead_code)]
struct DebugPrintable(f32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub(super) fn debug() {
    // {:?} print types that implements Debug in raw
    println!("--{:?} months in a year.", 12);
    println!(
        "--{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // Print Structure
    println!("--Now {:?} will print", Structure(43));
    println!("--Now {:?} will print", Deep(Structure(53)));

    let name = "Hur";
    let age = 19;
    let hur = Person { name, age };

    // Pretty print
    println!("--{:#?}", hur);
}
