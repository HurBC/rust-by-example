use std::fmt::Display;

struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--({} {})\n--({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

pub(super) fn activity() {
    println!("-Tuples Activity:");

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("--Matrix:");
    println!("{}", matrix);
    println!("--Transpose:");
    println!("{}", transpose(matrix));
}
