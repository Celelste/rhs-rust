/*
make the program succeed and output x in one of two ways:
1 - make x mutable, and change the value
2 - reassign x

*/

fn main() {
    ??? mut x = 10; // assign 10 to x



    let y = x + 5; // do not change this line

    ? += 5;

    // change nothing below
    assert_eq!(x, y); // will fail the program if x != y
    println!("{x}")
    println!("Success!");
}