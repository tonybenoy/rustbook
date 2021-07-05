use std::collections::HashMap;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(3);
    print!("{}", v[1]);
    match v.get(3) {
        Some(t) => {
            println!("{}", t)
        }
        None => {
            println!("None found")
        }
    }
    let vect = vec![1, 2, 3, 4];
    let a = vect.get(7);
    println!("{:#?}", a);
    for i in &vect {
        println!("{}", i)
    }
    let mut vect2 = vect;
    for j in &mut vect2 {
        println!("{}", j);
        *j += 10;
        println!("{}", j);
    }
    for j in vect2 {
        println!("{}", j);
    }
    let venum = vec![People::Age(23), People::Name(String::from("Tony"))];

    let a = String::from("a");
    let b = "dog".to_string();
    let c = String::from("cat");
    let new_string = format!("{} {}-{}", a, b, c);
    println!("{}", new_string);
    for chr in new_string.chars() {
        println!("{}", chr);
    }
    let p = make_new_hashmap();
    let mut m = make_new_hashmap();
    println!("{:?}", p);
    print_hashmap(p);
    m.entry("team".to_string()).or_insert("red".to_string());
    print_hashmap(m);
}

enum People {
    Age(i32),
    Name(String),
}

fn make_new_hashmap() -> HashMap<String, String> {
    let mut person = HashMap::new();
    person.insert(String::from("name"), String::from("Tony"));
    person.insert(String::from("age"), 32.to_string());
    person
}
fn print_hashmap(hm: HashMap<String, String>) {
    for (k, v) in hm {
        println!("{} {}", k, v)
    }
}
