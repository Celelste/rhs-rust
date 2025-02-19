// create the same program as before, but use one variable

fn main() {
    let x: (bool, char, &str) = ; // create x tuple

    // change nothing below
    assert_eq!("true: z is the final letter of the alphabet", format!("{}: {} {}", x.0, x.1, x.2));
    println!("{}", format!("{}: {} {}", x.0, x.1, x.2));
    println!("Success!");
}