struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 60;
    println!(
        "BASIC: the area of the rectangle is {} sq pixels",
        area(width, height)
    );

    let rect = (30, 60);
    println!(
        "TUPLES: the area of the rectangle is {} sq pixels",
        area_tuple(rect)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    println!(
        "Struct: the area of the rectangle is {} sq pixels",
        area_struct(rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: Rectangle) -> u32 {
    rect.height * rect.width
}
