// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let num: i32 = 4;
    match num {
        1 => println!("Number One"),
        2 => println!("Number Two"),
        3 => println!("Number Three"),
        _ => println!("Number Who knows what"),
    }
}
