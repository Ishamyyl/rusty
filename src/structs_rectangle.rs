#![allow(unused)]

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 1,
        height: 100,
    };

    let biggest = r1.max(&r2);
    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", biggest);
}

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

    fn max(&self, other: &Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
