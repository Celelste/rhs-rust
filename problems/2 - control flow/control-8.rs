// for this quiz, your program needs to be able to compute the first 10 fibonacci numbers for any given number, store them as an array, and print out that array
// ex input = 6: 0 6 6 12 18 30 48 78 126 204
//               ^ first number is always 0, to make programming easier
// since we haven't covered input, just change a var as your input for now


fn main() {
    let input: i32 = 6;
    let sequence: [i32; 10] =
    // coding space



    // change nothing below
    assert_eq!(sequence, fibonacci(input));
    println!("{:?}", sequence);
    println!("Success!")
}
pub mod mods {pub mod fibonacci;} // create mods module which contains fibonacci
use mods::fibonacci::fibonacci; // import fibonacci module, please don't go looking for this