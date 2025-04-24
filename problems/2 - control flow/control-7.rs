// Print out every letter of the alphabet by ONLY replacing ?'s

use std::ops::{Range}; // importing range so we can refer to it as a type

fn main() {
    let range: Range<char> = '?'..'?'; // print out letters a-z (letter after z is {)


    let mut sum: i32 = 0;
    for x in range {
        println!("{}", x);

        sum += x as i32; // do not change
    }

    // change nothing below
    assert_eq!(sum, 2847);
    println!("adding all the letters of the alphabet together gets {}!", sum);
}