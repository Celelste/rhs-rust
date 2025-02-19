// fill in the unknowns, and make the program succeed

fn main() {
    let x: bool = ???;

    let y: char = ???;

    let statement: &str = "is the final letter of the alphabet";

    // change nothing below
    println!("{x}: {y} {statement}");
    assert_eq!("true: z is the final letter of the alphabet", format!("{x}: {y} {statement}"));
    println!("Success!");
}