fn main() {
    let mut x = 5; //mutable variable
    println!("{}", x);
    x = x + 5; //changing the value of a mutable variable
    println!("{}", x);
    let y = 3; //immutable variable
    println!("{}", y);
    const Z: u32 = 54; // constant variable
    println!("{}", Z);
    let a: u32 = Z;
    let a = 5; //shadowing
    println!("{}", a);
}
