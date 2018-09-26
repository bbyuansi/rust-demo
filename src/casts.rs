#![allow(overflowing_literals)]

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn casts() {
    let decimal = 65.324_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("{}, {}, {}", decimal, integer, character);

    let nano: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} + {} = {}", nano, inches, nano + inches);
}