#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    phone: String,
}

#[derive(Debug)]
struct Structure(i32, i64);

fn print_debug() {
    let user = User { name: "biyuansi".to_string(), age: 12, phone: "13322211133".to_string() };
    let structure = Structure(10, 44);
    println!("{:#?}", user);
    println!("{:#?}", structure);
}