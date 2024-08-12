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

// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     fn freezing() -> Self {
//         Self { degrees_f: 32.0 }
//     }

//     fn boiling() -> Self {
//         Self { degrees_f: 123.5 }
//     }

//     fn show_temp(&self) {
//         println!("{:?} degrees_f", self.degrees_f)
//     }
// }

// fn main() {
//     let hot = Temperature { degrees_f: 99.9 };
//     let cold = Temperature::freezing();
//     let boil = Temperature::boiling();
//     hot.show_temp();
//     cold.show_temp();
//     boil.show_temp();
// }

// * Use an enum for the box color
enum BoxColor {
    Red,
    Blue,
}
impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Red => println!("red"),
            BoxColor::Blue => println!("blue"),
        }
    }
}
// * Use a struct to encapsulate the box characteristics
struct Dimensions {
    width: f64,
    depth: f64,
    height: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("depth: {:?}", self.depth);
        println!("height: {:?}", self.height);
    }
}
struct ShippingBox {
    weight: f64,
    color: BoxColor,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, BoxColor::Red, small_dimensions);
    small_box.print();

}