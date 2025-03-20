use std::fmt;

mod testcase;

#[allow(dead_code)]
struct Structure(i32);

// Display allow us use {} to print our structs, but we need to implement it manually
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub(super) fn display() {
    let minmax = MinMax(8, 14);

    println!("--Compare structs:");
    println!("--Display: {}", minmax);
    println!("--Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "--The big range is {big} and the small is {small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("--Compare points:");
    println!("--Display: {}", point);
    println!("--Debug: {:?}", point);

    let complex = Complex {
        imag: 7.2,
        real: 3.3,
    };

    println!("--Compare complex:");
    println!("--Display: {}", complex);
    println!("--Debug: {:?}", complex);

    println!("--Test Case:");
    testcase::test_case();
}
