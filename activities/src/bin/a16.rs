// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// struct Survey {
//     q1: Option<i32>,
//     q2: Option<bool>,
//     q3: Option<String>,
// }

// fn main() {
//     let response = Survey {
//         q1: Some(21),
//         q2: None,
//         q3: Some("A".to_owned()),
//     };
//     match response.q1 {
//         Some(ans) => println!("q1: {:?}", ans),
//         None => println!("q1: no response"),
//     }
//     match response.q2 {
//         Some(ans) => println!("q2: {:?}", ans),
//         None => println!("q2: no response"),
//     }
//     match response.q3 {
//         Some(ans) => println!("q2: {:?}", ans),
//         None => println!("q2: no response"),
//     }
// }

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: None,
    };
    println!("student: {:?}", mary.name);
    match mary.locker {
        Some(num) => println!("locker: {:?}", num),
        None => println!("no locker assigned"),
    }
}