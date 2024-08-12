// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// struct LineItem {
//     name: String,
//     count: i32,
// }

// fn print_name(name: &str) {
//     println!("name: {:?}", name);
// }

// fn print_count(count: i32) {
//     println!("count: {:?}", count)
// }

// fn main() {
//     let receipt = vec! [
//         LineItem {
//             name: "cereal".to_owned(),
//             count: 3,
//         },
//         LineItem {
//             name: String::from("fruit"),
//             count: 2,
//         },
//     ];

//     for item in receipt {
//         print_name(&item.name);
//         print_count(item.count);
//     }
// }


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String


// * Use an if expression to determine which person's info should be printed


struct Person {
    name: String,
    age: i32,
    color: String,
}
// * The name and colors should be printed using a function
fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec! [
        Person {
            name: String::from("Mark"),
            age: 32,
            color: String::from("green"),
        },
        Person {
            name: String::from("Jason"),
            age: 7,
            color: String::from("pink"),
        },
        Person {
            name: String::from("Sophie"),
            age: 9,
            color: String::from("blue"),
        },
    ];
    // * Iterate through the vector using a for..in loop    
    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
        }
    }
}