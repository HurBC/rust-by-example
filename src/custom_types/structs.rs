#![allow(dead_code)]

// C struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// Tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(self) -> f32 {
        let Rectangle {
            top_left: Point { x: tx, y: ty },
            bottom_right: Point { x: bx, y: by },
        } = self;

        let width = if tx > bx { tx - bx } else { bx - tx };
        let height = if ty > by { ty - by } else { by - ty };

        width * height
    }
}

fn square(point: Point, size: f32) -> Rectangle {
    let Point { x, y } = point;

    Rectangle { top_left: point, bottom_right: Point {
        x: x + size,
        y: y + size,
    } }
}

pub(super) fn structs() {
    let name = "Hur".to_string();
    let age = 19;

    let hur = Person { name, age };

    println!("--Person: {:?}", hur);

    // Instantiate a struct with fields
    let point = Point { x: 10.3, y: 0.4 };
    let another_point = Point { x: 2.3, y: 5.2 };

    // Accessing to point values
    println!("--point.x: {}", point.x);
    println!("--point.y: {}", point.y);

    // Instantiate a struct using other struct values
    let bottom_right = Point { x: 5.2, ..point };

    println!("--bottom_right.x: {}", bottom_right.x);
    println!("--bottom_right.y: {}", bottom_right.y);

    // Structs can be destructured to create bindings
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: another_point,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructured tuple Struct
    let Pair(integer, decimal) = pair;

    println!("--Pair integer: {integer}");
    println!("--Pair decimal: {decimal}");

    println!("--Rectangle: {:?}", rectangle);
    println!("--Rectangle Area: {:.1}", rectangle.rect_area());

    println!("--Square: {:?}", square(point, 5.0));
}
