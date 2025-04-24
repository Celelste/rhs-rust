// print out numbers 1-10 by ONLY changing unknowns to succeed

fn main() {
    let mut x: i32 = 0;

    ??? x != 10 { // while x != 10

        let y: i32 = x;

        ???; // increment x by 1

        // change nothing below
        assert_ne!(y, x); // make sure that x and y are different
        println!("{}", x);
    }

    assert_eq!(x, 10); // make sure that x == 10
    println!("Success!");
}