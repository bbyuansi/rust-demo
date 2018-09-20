use std::fmt::{self, Formatter, Display, Result};

struct City {
    name: &'static str,
    lon: f32,
    lat: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lon_c = if self.lon >= 0.0 { 'N' } else { 'S' };
        let lat_c = if self.lat >= 0.0 { 'W' } else { 'E' };
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lon.abs(), lon_c, self.lat.abs(), lat_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Colour {{ red: 0x{:02X}, green: 0x{:02X}, blue: 0x{:02X} }}",
               self.red, self.green, self.blue)
    }
}

fn format() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }

    for colour in [
        Colour { red: 128, green: 255, blue: 90 },
        Colour { red: 0, green: 3, blue: 254 },
        Colour { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *colour);
    }
}