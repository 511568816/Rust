use std::io;

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {

    fn area(&self) ->u32 {
        self.height * self.width
    }

    fn is_bigger_than(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }

    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle {
            height,
            width,
        }
    }
}

fn main() {
    let err_str = "_Error_";
    let mut width = String::new();
    let mut height = String::new();
    io::stdin().read_line(&mut width)
        .expect(err_str);
    io::stdin().read_line(&mut height)
        .expect(err_str);
    let width: u32 = width.trim().parse().expect(err_str);
    let height: u32 = height.trim().parse().expect(err_str);
    let rec1 = Rectangle::new(width, height);
    println!("rec1 area = {}", rec1.area());
    let rec2 = Rectangle::new(20, 30);
    println!("rec2 area = {}", rec2.area());
    if rec1.is_bigger_than(&rec2) {
        println!("rec1 is bigger than rec2");
    } else {
        println!("rec1 isn't bigger than rec2");
    }
}
