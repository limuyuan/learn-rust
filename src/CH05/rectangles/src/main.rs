fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let struct_rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area_by_struct(&struct_rect1));

    println!("rect1  = {:#?}", struct_rect1);
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
