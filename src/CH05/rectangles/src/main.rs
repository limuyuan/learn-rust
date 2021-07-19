fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let struct_rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", struct_rect1.area());

    println!("rect1  = {:#?}", struct_rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 29,
        height: 51,
    };

    println!("Can rect1 hold rect2? {}", struct_rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&struct_rect1));
    println!("can rect1 hold rect3? {}", struct_rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
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

}
