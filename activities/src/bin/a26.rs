// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::prelude::*;

fn main() {

    let gateway: DateTime<Local> = Local::now();
    println!("{}", gateway.format("%Y-%m-%d %H:%M").to_string());

    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%Y-%m-%d %H:%M:%S").to_string());

}
