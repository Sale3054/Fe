#[derive(Debug)]
struct Rectangle {
    height: u64,
    width: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height >= other.height)
            || (self.width >= other.width && self.height > other.height)
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        height: 60,
        width: 100,
    };

    dbg!(&rect);
    println!("Area of rect: {}", &rect.area());

    let rect2: Rectangle = Rectangle {
        height: 50,
        width: 50,
    };

    println!("{}", rect.can_hold(&rect2));
}
