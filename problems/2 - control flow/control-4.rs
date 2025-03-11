// print out numbers 1-10 by ONLY changing unknowns to succeed

fn main() {
    let mut x: i32 = 0;

    loop {
        ??? x == 10 { // if x == 10 break the loop
            ???;
        }

        let y: i32 = x;

        x += 1;

        assert_ne!(y, x);
        println!("{}", x);
    }

    assert_eq!(x, 10);
    println!("Success!");
}