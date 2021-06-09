#![allow(dead_code)]
#![allow(unused_variables)]
// Structs and enums, yay

// struct with named data
struct Rect {
    a: f32, b: f32, x: f32, y: f32, color: Color,
}

// struct with unnamed data, tuple-like
#[derive(Debug)]
struct Vector (f32, f32);

// struct with name but no data
#[derive(Debug)]
struct NamedStruct;

// enum: holds fields, accessible by enum's name
#[derive(Debug)]
enum Shapes {
    Square, Rectangle, Circle,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn getsize(rect: &Rect) -> f32 {
    return rect.a * rect.b;
}

fn printshape(shape: Shapes) {
    // match variable by it's value (basically switch)
    match shape {
        Shapes::Square => println!("Shape is a square"),
        Shapes::Rectangle => println!("Shape is a rectangle"),
        Shapes::Circle => println!("Shape is a circle"),
    }
}

fn main() {

    let r1 = Rect {a: 4f32, b: 2f32, x: 0f32, y: 0f32, color: Color::Red};
    let r2 = Rect {a: 3f32, b: 6f32, x: 0f32, y: 0f32, color: Color::Blue};

    println!("Rectangle 1 size: {}, color: {:?} / {}", getsize(&r1), r1.color, r1.color as i32);
    println!("Rectangle 2 size: {}, color: {:?} / {}", getsize(&r2), r2.color, r2.color as i32);

    let shape1 = Shapes::Square;
    let shape2:Shapes = Shapes::Rectangle;

    println!("Shape 1 is: {:?}", shape1);

    printshape(shape1);
    printshape(shape2);

    let v = Vector(0.5f32, 0.3f32);
    println!("Vector size is: x: {}, y: {}", v.0, v.1);
    println!("Vector itself: {:?}", v);
    
    let ns = NamedStruct;
    println!("Struct with name but no data: {:?}", ns);

    println!("Shapes numeric values: square: {}, rectangle: {}, circle: {}", Shapes::Square as i32, Shapes::Rectangle as i32, Shapes::Circle as i32);

}
