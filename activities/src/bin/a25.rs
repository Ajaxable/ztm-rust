// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::all_caps;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from ("HELLO");
        assert_eq!(result, expected, "string should be all uppercase");
    }

}
