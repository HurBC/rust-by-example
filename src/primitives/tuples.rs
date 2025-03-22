fn reverse(pairs: (i32, bool)) -> (bool, i32) {
    // Tuples can be destructured to create bindings
    let (integer, boolean) = pairs;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub(super) fn tuples() {
    // Tuples can have multiple types inside them
    // Their can be his own type
    let long_tuple = (
        // integer from 8 to 128 bits
        1u8,
        // str
        "Hello",
        // Strings
        "World".to_string(),
        // Other Tuples
        (1, true),
        // Struct
        Matrix(1.1, 1.2, 2.1, 2.2),
        // Arrays
        ['H', 'e', 'l', 'l', 'o'],
        // etc
    );

    println!("-long_tuple: {:#?}", long_tuple);

    // Tuples index start at 0 && can be accessed with tuple_name.index
    println!("-long_tuple.0: {}", long_tuple.0);
    println!("-long_tuple.1: {}", long_tuple.1);

    let pair: (i32, bool) = (1, true);

    println!("-pair: {:?}", pair);
    println!("-reverse(pair): {:?}", reverse(pair));

    // To create a tuple must be add a comma to distinguish it from a literal
    println!("-One element tuple: {:?}", (5u32,));
    println!("-Just a integer: {:?}", (5u32));

    super::tuples_activity::activity();
}
