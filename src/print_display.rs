use std::fmt;

#[derive(Debug)]
struct Numbers(i32, i64);

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},X {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Structure {
    name: String,
    age: i32,
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, age: {}", self.name, self.age)
    }
}

fn print_display() {
    let num = Numbers(1, 2);
    println!("--------------");
    println!("{}", num);
    println!("{:?}", num);
    println!("-----------");
    let strt = Structure { name: "biyuansi".to_string(), age: 20 };
    println!("{}", strt);
    println!("{:?}", strt);
}
