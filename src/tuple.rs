use std::fmt::{self, Formatter, Display, Result};

#[derive(Debug)]
struct Matrix(i32, u32, f32, f64);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "({}, {})", self.0, self.1);
        writeln!(f, "({}, {})", self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn tuple() {
    let pair = (1, true);
    println!("{:?}", pair);
    println!("{:?}", reverse(pair));

    let tuple = (1, true, 1f32, "hello");
    println!("{:#?}", tuple);
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1i32, 3u32, 4f32, 5f64);
    println!("{:?}", matrix);

    let mt = Matrix(1, 100u32, 1000f32, 10000f64);
    println!("{}", mt);
}