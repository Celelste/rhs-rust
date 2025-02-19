/*
complete the test_val function to succeed:
1 - test_val outputs true if x is even
2 - test_val outputs false if x is odd, or is either 24 or 42
*/

fn main() { // change nothing in fn main {}
    // 'true' statements
    assert_eq!(test_val(2), true);
    assert_eq!(test_val(36), true);
    assert_eq!(test_val(44), true);
    assert_eq!(test_val(1000), true);
    // 'false' statements
    assert_eq!(test_val(1), false);
    assert_eq!(test_val(5), false);
    assert_eq!(test_val(23), false);
    assert_eq!(test_val(24), false);
    assert_eq!(test_val(42), false);
    assert_eq!(test_val(1001), false);

    println!("Success!");
}

// test_val outputs true if x is even
// test_val outputs false if x is odd, or is either 24 or 42

fn test_val(x: i32) -> bool { // ignore for now, we will cover how functions work later



    return return_bool; // do not change line
}