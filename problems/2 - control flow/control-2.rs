// Practical If-else Statements

// fill in the ?'s to output the correct weather statement for the given temperature

fn main() {
    let temperature = 75; // change this value to test different outputs
    let temp_state: &str; // declare a variable to hold the weather state

    ??? temperature < 30 { // lower than 30 is very cold
        println!("It's very cold outside");
        temp_state = "very cold";
    } else if temperature < 50   { // lower than 50 is cold
        println!("It's a bit chilly");
        temp_state = "cold";
    } ??? temperature < 80 { // lower than 80 is pleasant
        println!("The weather is pleasant");
        temp_state = "pleasant";
    } ??? { // higher than 80 is hot
        println!("It's hot outside");
        temp_state = "hot";
    }

    // change nothing below
    assert_eq!(weather(temperature), temp_state);
    println!("success!");
}

// Ignore all below
pub mod mods {pub mod weather;}
use mods::weather::weather;