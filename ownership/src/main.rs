fn main() {
    let mut str = String::from("Hello");
    str.push_str(" world");
    println!("{}", str);
    let s2 = str;
    let s3 = s2.clone();
    println!("{} {}", s2, s3);
    take_ownership(s3);
    let g = give_onwnership();
    println!("{}", g);
    let x = sharing_is_caring(g);
    println!("{}", x);
    let k = referencing(&x);
    println!("{} {}", k, x);
    let mut w = String::from("Hello");
    mut_reference(&mut w);
    println!("{}", w);
    let s = String::from("hello world");
    let a = slicestring(s);
    println!("{}", a.0)
}

fn mut_reference(s: &mut String) {
    s.push_str(" Well played")
}

fn take_ownership(x: String) {
    println!("{}", x)
}

fn give_onwnership() -> String {
    let str = String::from("Tony");
    str
}

fn sharing_is_caring(y: String) -> String {
    y
}

fn referencing(s: &String) -> usize {
    s.len()
}

fn slicestring(s: String) -> (String, String) {
    let hello = &s[..5];
    let world = &s[6..];
    return (hello.to_string(), world.to_string());
}
