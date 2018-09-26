static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn constants() {
    let n = 16;
    println!("this is {}", LANGUAGE);
    println!("{}", THRESHOLD);
    println!("{}, {}", n, if is_big(n) { "big" } else { "small" });
}