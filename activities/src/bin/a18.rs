// Topic: Result
//
// Requirements:

//   * Implement Debug print functionality using `derive`
// * Create an structure named `Adult` that represents a person aged 21 or older:
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}
//   * The structure must contain the person's name and age
// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }
    //   * The Err variant should contain a String (or &str) that explains why
    //     the structure could not be created
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    let child = Adult::new(15, "Anita");
    //   * One should be 21 or over
    let adult = Adult::new(21, "Sanjay");
    // * Use `match` to print out a message for each `Adult`:
    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}"),
    }
    match adult {
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        Err(e) => println!("{e}"),
    }
}


//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

