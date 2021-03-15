#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    }
    impl Rectangle {
        fn perimeter(&self) -> u32 {
            let out = self.width+self.height;
            out *2

        }
        }
    fn main() {
    let mut rect1 = Rectangle {
    width: 30,
    height: 50,
    };
    println!(
    "The area of the rectangle is {} square pixels and perimeter is {:#?}",
    area(&rect1),&rect1.perimeter()
    );
    }
    fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    }
