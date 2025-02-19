/*
for each number in data, determine if it is a prime
if number is prime print "_ is prime"
if number isn't prime, print "_ is not a prime. it's factors are: [_]
*/

fn main() {
    let data: [i32; 5] = [51, 24, 39, 12, 73];

    let mut x: usize = 0; // usize is an unsigned integer of unknown size

    while x < data.len() { // len returns the length of the given array, in this case 5

        let val = data[x];

        if is_prime(val) {
            println!("{} is prime", val);
        } else {
            println!("{} is not prime, its factors are: {:?}", val, factors(val));
        }
        x += 1

    }
}

fn is_prime(n: i32) -> (bool) { // this function returns a boolean
    let mut x: i32 = 0;
    while x < n {
        x += 1;
        if (n % x == 0) & (x != 1) & (x != n) {
            return false;
        }
    }

    true
}

fn factors(n: i32) -> Vec<i32> {
    // this function returns an array of unknown length, called a vector.
    // for now, assume that they function the same as regular arrays.

}