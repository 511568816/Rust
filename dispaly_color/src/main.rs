use std::fmt;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) ", self.red, self.green, self.blue);
        write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 一旦添加了针对 fmt::Display 的实现，则要用 {} 对输出内容进行转换
        println!("{}", color)
    }
}
