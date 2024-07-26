// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
enum Flavor {
    Sweet,
    Tart,
    Bitter,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Cake {
    flavor: Flavor,
    gram_weight: i32,
}
// * Use a function to print out the drink flavor and ounces
fn print_cake(cake: Cake){
    match cake.flavor {
        Flavor::Sweet => println!("flavor: Sweet"),
        Flavor::Tart => println!("flavor: Tart"),
        Flavor::Bitter => println!("flavor: Bitter")
    }
    println!("cake size: {:?}", cake.gram_weight);
}
// * Use a match expression to print the drink flavor
fn main() {  
    let lemon = Cake {
        flavor: Flavor::Sweet,
        gram_weight: 60,
    };
    let carrot = Cake {
        flavor: Flavor::Tart,
        gram_weight: 120,
    };

    print_cake(carrot);
    print_cake(lemon);
}
        

// struct ShippingBox {
//     depth: i32,
//     width: i32,
//     height: i32,
// }


// fn main() {
//     let my_box = ShippingBox {
//         depth: 3,
//         width: 2,
//         height: 5,
//     };
//     let tall = my_box.height;
//     println!("the box is {:?} units tall", tall)
// }
// struct GroceryItem {
//     stock: i32,
//     price: f64,
// }

// fn main() {
//     let cereal = GroceryItem {
//         stock: 10,
//         price: 2.99,
//     };
//     println!("stock: {:?}", cereal.stock);
// }