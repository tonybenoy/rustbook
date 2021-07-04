fn main() {
    let a: i32 = 10; //32-bit integer
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("{}", c);
    let f: f64 = 32.1 + 54.0; //64-bit floating point
    println!("{}", f);
    let bo: bool = false; //bool
    println!("{}", bo);
    let character: char = 'a'; //charecter
    println!("{}", character);
    let tup: (i32, f64, u8) = (500, 6.4, 1); //tuple
    println!("{}", tup.0);
    let (x, y, z) = tup; //destructuring a tuple
    println!("{}", x);
    let q: [i32; 4] = [1, 2, 3, 4]; //list
    println!("{}", q[1]);
    let g = [3; 5];
    println!("{}", g[1])
}
