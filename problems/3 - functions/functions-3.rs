// for this quiz, create the change_number function to succeed

fn main() { // change nothing in main
    let x = 5;
    let y = x.clone();
    change_number(x); // this function takes a number and returns a different number
    assert_ne!(x, y);
    println!("success!");
}

