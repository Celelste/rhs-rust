// fill in the ?'s to make the code compile and succeed

fn main() { // the main function is complete, do not change
    let input_list = [51, 24, 39, 12, 73];
    let mut output_list = [false; 5];

    for (idx, num) in input_list.iter().enumerate() { // iter() returns an iterator over the elements of the array, enumerte() returns an iterator that yields pairs of the index and the value
        if is_even(num) { //take the number and pass it to the function is_even
            output_list[idx] = true;
        } else {
            output_list[idx] = false;
        }
    }

    for (idx, num) in input_list.iter().enumerate() { // for all the numbers in the input list print the number and if it is even or not
        if output_list[idx] {
            println!("{} is even", num);
        } else {
            println!("{} is not even", num);
        }
    }

    println!("success!");
}

// when declaring a function, we need to specify the name of the input, its type, and the return type
// fn function_name(input: type) -> return_type { // this is the syntax for declaring a function
fn is_even(???) -> ??? { // this function takes a number, and returns a boolean (number is i32, boolean is bool)
    if num % 2 == 0 { // if the number is even
        true // because this is the last line of the function we need to run, we can omit the ; and it will return it instead of running code
    } else {
        false
    }
}