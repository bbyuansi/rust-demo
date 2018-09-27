#[allow(dead_code)]

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32),
}

fn main() {
    let number = 100;
    match number {
        1 => println!("one"),
        2 | 3 | 9 | 100 => println!("one hundred"),
        _ => println!("no match"),
    }

    let pair = (1, 100);
    match pair {
        (1, x) => println!("1{}", x),
        (y, 100) => println!("2{}", y),
        _ => println!("hehe"),
    }

    let color = Color::RGB(100, 34, 244);
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("{}, {}, {}", r, g, b),
        Color::HSL(h, s, l) => println!("{}, {}, {}", h, s, l),
        Color::HSV(h, s, v) => println!("{}, {}, {}", h, s, v),
        _ => println!("hehe")
    }
}