fn main() {
    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("{:?}", home);
    let coin = Coin::Penny;
    println!("{:?}", coin);
    let resp = valin(coin);
    println!("{}", resp);
    let coin = Coin::Nickel;
    println!("{:?}", coin);
    let resp = valin(coin);
    println!("{}", resp);
    let r = func(Some(23));
    println!("{:?}", r);
    let r = func(None);
    println!("{:?}", r);
    let g = 22;
    match g {
        21 => println!("{}", "21"),
        22 => println!("{}", "22"),
        _ => println!("{}", "Zilch"),
    }
}

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valin(coin: Coin) -> String {
    match coin {
        Coin::Penny => String::from("Penny"),
        Coin::Nickel => {
            println!("Nickel is the way");
            String::from("Nickel")
        }
        Coin::Quarter => String::from("Quarter"),
        Coin::Dime => String::from("Dime"),
    }
}

fn func(op: Option<i32>) -> Option<i32> {
    match op {
        None => None,
        Some(x) => Some(x + 1),
    }
}
