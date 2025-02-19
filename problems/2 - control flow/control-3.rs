// print out numbers 1-10 by ONLY changing unknowns to succeed

fn main() {
    let mut x: i32 = 0;

     ??? ??? {
         let y: i32 = x;

         ???;

         assert_ne!(y, x);
         println!("{}", x);
     }

    assert_eq!(x, 10);
    println!("Success!");
}