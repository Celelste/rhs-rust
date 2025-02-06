// make the program output x in one of two ways:
// 1 - reassign x
// 2 - make x mutable, and change the value

fn main() {
    let x = 10; // assign x



    let y = x + 5; // do not change this line



    // change nothing below
    assert_eq!(x, y); // will fail the program if x != y
    println!("{x}")
}