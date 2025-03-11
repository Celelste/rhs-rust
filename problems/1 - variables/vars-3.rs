/*
make the program compile and succeed:
1 - change x to a float
2 - convert y into an int
*/

fn main() {
    let mut x: i32 = 5; // x = 5 as an integer
    let y: f64 = 5.0; // do not change, y = x as a float

    x += y as ???;

    println!("{}", x);
    println!("Success!");
}