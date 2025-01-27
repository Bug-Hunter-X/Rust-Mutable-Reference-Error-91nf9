fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y (and indirectly to x)
    *y = 10;  // Modifying x through y is allowed
    println!("x = {}", x); // Prints x = 10
    // let a = &mut x; // This line would cause a compile-time error because a and y are both mutable references to x
}