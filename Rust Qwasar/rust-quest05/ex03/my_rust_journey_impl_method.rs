#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Rectangle 1: {:?}", rect1);
    println!("Rectangle 1 area {}", rect1.area());

    println!("Can Rectangle #1 hold Rectangle #2? {}", rect1.can_hold(&rect2));
    println!("Can Rectangle #1 hold Rectangle #3? {}", rect1.can_hold(&rect3));
}
