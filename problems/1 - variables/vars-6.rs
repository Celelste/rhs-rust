// Fill in ONLY ?'s to make program compile and succeed

fn main() {
    let mut num_array: ? = [?]; // should be 1, 2, 3, 4, 5
    let fives: ? = [?; ?]; // should be 5, 5, 5, 5, 5


    assert_eq!(num_array.iter().sum(), 14); // do not change line

    // fix error in num_array so total = 15



    // change nothing below
    let mut num_sum: i32 = 0;
    let mut five_sum: i32 = 0;
    for num in num_array.iter() { // iterate through each number
        num_sum += num;
    }
    five_sum = fives.iter().sum(); // same thing, written more conveniently

    assert_eq!(num_sum, 15);
    assert_eq!(five_sum, 25);

    println!("{:?} - {num_sum}", num_array);
    println!("{:?} - {five_sum}", fives);
    println!("Success!");
}