// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Green,
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Green => println!("Green"),
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
        }
    }
}

struct Dimensions {
    width: i32,
    height: i32,
    depth: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct Box {
    weight: i32,
    color: Color,
    dimensions: Dimensions,
}

impl Box {
    fn new(weight: i32, color: Color, dimensions: Dimensions) -> Box {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight {:?}", self.weight);
    }
}
fn main() {
    let d = Dimensions {
        width: 2,
        height: 4,
        depth: 6
    };
    let new_box = Box::new(2, Color::Green, d);
    new_box.print()
}
