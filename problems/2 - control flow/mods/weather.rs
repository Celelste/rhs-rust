pub fn weather(temperature: i32) -> &'static str {

    if temperature < 30 { // lower than 30 is very cold
        "very cold"
    } else if temperature < 50 { // lower than 50 is cold
        "cold"
    } else if temperature < 80 { // lower than 80 is pleasant
        "pleasant"
    } else { // higher than 80 is hot
        "hot"
    }
}