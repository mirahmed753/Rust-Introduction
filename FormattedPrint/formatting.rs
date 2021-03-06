use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// create an implementation for Color
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let red_hex = format!("{:02}{:X}", 0, self.red);
        let green_hex = format!("{:X}", self.green);
        let blue_hex = format!("{:X}", self.blue);

        // check if values need to be padded with zeroes


        // open paren
        write!(f, "RGB (")?;

        // color numeric values
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)?;

        // close paren
        write!(f, ") ")?;

        // hex value
        write!(f, "0x{}{}{}", red_hex, green_hex, blue_hex)
    }
}
// write!(f, "0x{}{}{}", format!("{:02}", red_hex), format!("{:02}", green_hex), format!("{:02}", blue_hex)

// println!("Hello {:width$}!", "x", width = 5);

fn main() {
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
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{}", *color);
    }
}
