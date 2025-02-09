fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x;
        *y += 1;
    }
    { // Create another new scope
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
//Alternative solution using cloning:
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y += 1;
    z += 1; 
    x = y + z -10; // Note that this uses x = y + z - 10 to adjust values so that x = 6.  This is a simplified example only.
    println!("x = {}", x);
} 