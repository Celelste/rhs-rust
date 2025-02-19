/*
make the program compile and succeed:
1 - change x to a float
2 - convert y into an int
*/

fn main() {
    let mut x = 5;
    let y = 5.0; // do not change

    x += y;

    println!("{}", x);
    println!("Success!");
}