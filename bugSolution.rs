fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Prints x = 6

    // Correct way to handle immutable references:
    let z = &x;  
    println!("x = {}", *z); // Prints x = 6
} 