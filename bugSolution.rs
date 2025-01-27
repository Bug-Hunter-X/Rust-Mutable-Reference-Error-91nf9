fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    } // y goes out of scope here.  Now it is safe to create another mutable reference.
    let a = &mut x; //This is now allowed because y is no longer in scope
    *a = 15;
    println!("x = {}", x); // Prints x = 15
} 