#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect_one = Rectangle { width: 30, height: 50 };

    println!("rect_one is {:?}", rect_one);
    println!("The area of the rectangle is {:?}", area(&rect_one));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
