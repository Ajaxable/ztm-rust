// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let ticket_holders = vec! [ 
        Ticket::Backstage(52.50, "Bobby John".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Amy".to_owned()),
    ];

    for ticket in ticket_holders {
        match ticket {
            Ticket::Backstage(price, holder) => println!("Backstage: ${:?}, name: {:?}",price, holder),
            Ticket::Standard(price) => println!("Standard: ${:?}", price),
            Ticket::Vip(price, holder) => println!("VIP: ${:?}, Name: {:?}", price, holder),
        }
    }

}







// enum Discount {
//     Percent(i32),
//     Flat(i32),
// }
// struct Ticket {
//     event: String,
//     price: i32,
// }

// fn main() {
//     let n = 3;
//     match n {
//         3 => println!("three"),
//         other => println!{"number: {:?}", other},
//     }
//     let flat = Discount::Flat(5);
//     match flat {
//         Discount::Flat(2) => println!("flat 2"),
//         Discount::Flat(amount) => println!("discount of {:?}", amount),
//         _ => (),

//     }
//     let concert = Ticket {
//         event: "concert".to_owned(),
//         price: 25,
//     };
//     match concert {
       
//         Ticket {price: 25, event} => println!("25er event = {:?}", event),
//         Ticket {price, ..} => println!("Price = {:?}", price),
//     }
// }
