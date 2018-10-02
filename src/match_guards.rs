fn match_guards() {
    let pair = (2, 100);
    match pair {
        (x, y) if x == y => println!("x == y"),
        (x, y) if x + y > 100 => println!(" x = y > 100"),
        (x, _) if x > 100 => println!("x > 100"),
        _ => println!("none"),
    }

    match age() {
        n if n < 50 && n > 1 => println!("hehe"),
        _ => println!("100"),
    }

    match age() {
        n @ 1...99 => println!("ehhe, {}", n),
        n @ 99...200 => println!("haha, {}", n),
        _ => println!("enen"),
    }
}

fn age() -> i32 {
    100
}