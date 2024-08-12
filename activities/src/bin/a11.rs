// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


// struct Book {
//     pages: i32,
//     ratings: i32,
// }

// fn disp_page_count(book: &Book) {
//     println!("pages = {:?}", book.pages);
// }

// fn display_rating(book: &Book) {
//     println!("ratings = {:?}", book.ratings);
// }

// fn main() {
//     let book = Book {
//         pages: 55,
//         ratings: 8,
//     };
//     disp_page_count(&book);
//     display_rating(&book);
// }




// enum Light {
//     Bright,
//     Dim,
// }

// fn display_light(light: &Light) {
//     match light {
//         Light::Bright => println!("bright"),
//         Light::Dim => println!("dim"),
//     }
// }

// fn main() {
//     let dull = Light::Dim;
//     display_light(&dull);
//     display_light(&dull);
// }

enum Grocery {
    Eggs,
    Cheese,
}
// * Use a struct for the grocery item
struct GroceryItem {
    name: Grocery,
    quantity: i32,
    id: i32
}
// * Use two i32 fields for the quantity and id number
fn display_name(item: &GroceryItem) {
    println!("Item name: {:?}", item.name)
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("Item stock = {:?}", item.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
    println!("Item id = {:?}", item.id);
}

fn main() {
    let eggs = GroceryItem {
        name: Grocery::Eggs,
        quantity: 40,
        id: 32,
    };
    let cheese = GroceryItem {
        name: Grocery::Cheese,
        quantity: 56,
        id: 12,
    };

    display_quantity(&eggs);
    display_id(&eggs);
}