#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect_one = Rectangle { width: 30, height: 50 };
    let rect_two = Rectangle { width: 10, height: 40 };
    let rect_three = Rectangle { width: 60, height: 45 };

    println!("rect_one is {:?}", rect_one);
    println!("The area of the rectangle is {:?}", rect_one.area());
    println!("Can rect_one hold rect_two? {}", rect_one.can_hold(&rect_two));
    println!("Can rect_one hold rect_three? {}", rect_one.can_hold(&rect_three));
}


