

fn main() {
    let mut counter = 1;
    'outer: loop {
        'inner: loop {
            counter = counter + 1;
            if counter >= 3 {
                break;
            }
            if counter >= 5 {
                break 'outer;
            }
        }
    }
    println!("{}", counter);
}