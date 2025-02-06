// create the same program as before, but use one variable

fn main() {
    let x: (bool, char, &str) = (true, 'z', "is the final letter of the alphabet"); // create x tuple

    // change nothing below
    let (bool, letter, statement) = x;
    let full_statement = format!("{bool}: {letter} {statement}");
    assert_eq!("true: z is the final letter of the alphabet", full_statement);
    println!(full_statement);
}