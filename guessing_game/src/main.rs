use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
    println!("Enter a number between 1 and 100");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal =>{ println!("You win!");
    break;
    },}
    println!("You entered: {}", guess);
}
}
