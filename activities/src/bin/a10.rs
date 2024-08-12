// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest,
// }

// fn main() {
//     // secret file: amins only
//     let access_level = Access::Guest;
//     let can_access_file = match access_level {
//         Access::Admin => true,
//                     _ => false,
//     };
    
//     println!("can access? {:?}", can_access_file);

// }
// * Use a function to print the messages
fn print_message(greater_than_100: bool) {
  // * Use a match expression to determine which message
//   to print  
    match greater_than_100 {
        true => println!("its big"),
        false => println!("its small")
    }
}



fn main() {
    // * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
    let value = 10;    
    let greater_than_100 = value > 100;
    print_message(greater_than_100);
}
