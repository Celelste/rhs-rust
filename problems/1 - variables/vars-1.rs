/*
make the program compile and succeed in two ways:
1 - declare x separately
2 - declare & assign in one line
*/

fn main() {
    let x ? ?; // assign 10 to x
    x = ?;

    // change nothing below
    println!("{x}"); // print x to terminal
    assert_eq!(x, 10);
    println!("Success!");
}