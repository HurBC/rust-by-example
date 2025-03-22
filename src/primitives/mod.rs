pub mod arrays_slices;
pub mod tuples;

mod tuples_activity;

pub fn primitives() {
    let logical = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable = true; // Error: expected integer, found boolean

    let mutable = true;

    let my_array = [1, 2, 3, 4, 5];

    let my_tuple = (5u32, 1u8, true, -5.04f32);

    tuples::tuples();
    arrays_slices::arrays_slices();
}
