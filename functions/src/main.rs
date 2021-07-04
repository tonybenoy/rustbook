fn main() {
    println!("main func!");
    let x = 5;
    func2(x, 7);
    let k = 43;
    if checkifgreaterthan10(k) {
        println!("k is greater than 10");
    }
    let e = if checkifgreaterthan10(k) { 10 } else { k };
    println!("{}", e);
    loopandprint(k);
    whilecanprint(k);
    iter_list();
    range(k);
}

fn func2(y: i32, b: i32) {
    println! {"func2 {}",y+b};
    let q = {
        let j = 8;
        j + 1
    };
    println!("{}", q);
    println!("{}", funccacl(q));
    println!("{}", checkeven(q));
}

fn funccacl(i: i32) -> i32 {
    i + 32
}

fn checkifgreaterthan10(x: i32) -> bool {
    if x > 10 {
        true
    } else {
        false
    }
}

fn checkeven(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x % 2 != 0 {
        1
    } else {
        2
    }
}

fn loopandprint(val: i32) {
    let mut counter = val;
    loop {
        counter = counter - 1;
        println!("{}", counter);
        if counter == 0 {
            break;
        }
    }
}
fn whilecanprint(val: i32) {
    let mut c = val;
    while c != 0 {
        println!("{}", c);
        c -= 1
    }
}

fn iter_list() {
    let a = [3; 5];
    for i in a.iter() {
        println!("{}", i);
    }
}

fn range(i: i32) {
    for j in (1..i) {
        println!("{}", j)
    }
}
