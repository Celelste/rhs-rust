
pub fn fibonacci(n: i32) -> [i32; 10] {

    let mut sequence = [0; 10];
    sequence[1] = n;
    for i in 1..9 {
        sequence[i + 1] = sequence[i] + sequence[i - 1];
    }
    sequence
}
