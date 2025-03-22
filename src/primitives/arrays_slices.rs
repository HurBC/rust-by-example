use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("-first element of the slice: {}", slice[0]);
    println!("-the slice has {} elements", slice.len());
}

pub(super) fn arrays_slices() {
    // Arrays are collections of the same type and a fixed length
    // The type and length are both part of the array's type signature
    let simple_arr = [1, 2, 3, 4, 5];

    // An Array can be initialized with the data type and his length
    let zeros_array = [0; 500];

    // As tuples, Arrays indexes start at 0 using array[index]
    println!("-simple_arr[0]: {}", simple_arr[0]);

    // As tuples, Arrays storage his data in the stack continuously

    println!("-size of simple_arr: {}", mem::size_of_val(&simple_arr));
    println!("-size of zeros_array: {}", mem::size_of_val(&zeros_array));

    // Arrays can be automatically borrowed as slices
    println!("-borrow the whole array as a slice");
    analyze_slice(&simple_arr);

    // Slices can point to a section of an array
    // Like Arrays. Slices can store only data of the same type but his length isn't know in compile time
    println!("-borrow a section of the array as a slice");
    analyze_slice(&simple_arr[1..4]);

    // Empty slice
    let empty_array: [u32; 0] = [];

    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // The elements of an array can be safety accessed with the .get method, this method returns a Option value

    // This will be return a None value because the index is out of bounds
    println!("-{:?}", simple_arr.get(5));

    // This will be a panic! because the index is out of bounds
    // println!("{}", simple_arr[5]);
}
