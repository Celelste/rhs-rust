fn main() {
    // optional experimentation here
    println!("--take ownership--");
    take_ownership();
    println!("--borrow value--");
    borrow_value();
    println!("--copy value--");
    copy_value();
    println!("--types of heap vars--");
    types_of_heap_vars();

}

fn take_ownership() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // this would cause a compile-time error since "hello" is owned by s2 now
    println!("{}", s2); // this is fine since s2 owns "hello"
    drop(s2); // s2 is dropped, and "hello" is deallocated
    // s1 practically does not exist anymore, so deleting s2 would also delete "hello", and s1 holds no value
    // println!("{:?}, {:?}", s1, s2) // "hello" no longer exists in either s1 or s2
}

fn borrow_value() {
    let mut s1 = String::from("hello"); // mutable values can still be borrowed
    let r1 = &s1; // borrow s1 - this is a reference, and cannot change the value
    // r1 = &String::from("world"); // this would cause a compile-time error since r1 is immutable
    println!("{}", r1); // use r1
    let r2 = &mut s1; // borrow s1 mutably - this is a mutable reference
    r2.push_str(", world!"); // modify s1 through r2
    println!("{:?}", s1); // this is fine since s1 is still valid
}

fn copy_value() {
    let x = 5; // x is an integer, which is a Copy type
    let mut y = x; // y is a copy of x
    // x and y have the same value, but they are different variables (changing one does not change the other)
    println!("{:?}, {:?}", x, y); // this is fine since x and y are both valid
    y += 5; // modify y
    println!("{:?}", x); // this is fine since x is still valid, it is also unchanged
}

fn types_of_heap_vars() {
    let x: i32 = 5; // x is an integer of 32 bits, which is stack allocated
    let y: isize = 10; // y is an integer of unknown bits, which is heap allocated
    let z: Box<i32> = Box::new(20); // z is a heap allocated integer of 32 bits (box puts a value on the heap)
    // these are all practically the same, but y can be as large as the machine can handle

    let a: &str = "hello"; // a is a string slice, which is stack allocated
    let mut b: String = String::from("hello"); // b is a heap allocated string
    // a is a reference to a string literal, while b is a heap allocated string
    // a points to "hello" on the stack, it has a location and a length (this way you can use it as a slice)
    b.push_str(" world!"); // this is fine since b is mutable
    println!("{}", b); // this is fine since b is still valid

    let i: [i32; 5] = [1, 2, 3, 4, 5]; // i is an array of integers of 32 bits, which is stack allocated
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5]; // v is a vector of integers of 32 bits, which is heap allocated
    v.push(6); // this is fine since v is a mutable vector
    v.remove(2); // this is fine since v is a mutable vector
    // i is a fixed size array, while v is a dynamic size array meaning we can grow or shrink it
    println!("{:?}", v); // this is fine since b is still valid

}