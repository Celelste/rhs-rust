/*
If-else statement quiz

complete the test_val function to succeed:
1 - test_val outputs true if x is even
2 - test_val outputs false if x is odd, or is either 24 or 42

test_val outputs true if x is even
test_val outputs false if x is odd, or is either 24 or 42

x % 2 == 0 (x is even)
x % 2 != 0 (x is odd)
*/

fn test_val(x: i32) -> bool { // ignore for now, we will cover how functions work later

    ??? { // if 24 or 42, return false
        return false
    } ??? { // if even, return true
        return true
    } ??? { // else, return false
        return false
    }
}

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