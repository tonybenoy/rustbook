fn main() {
    let u1 = User {
        username: String::from("tony"),
        age: 99,
        active_user: true,
    };
    let u2 = make_user(String::from("tony"), 22);
    println!("{} {}", u1.username, u2.active_user);
    let u3 = User {
        username: String::from("TONY"),
        ..u1
    };
    println!("{}", u3.age);
    let c2 = Cordinates(1, 2, 3);
    let c1 = Cordinates(3, 5, 6);
    let c3 = Cordinates(5, 7, 6);
    print! {"{}",c2.0};
    println!("{}", calculateareaoftriangle(&c1, &c2, &c3));
    let rt = RightTriangle {
        base: findlenghtofline(&c1, &c2),
        hypotenuse: findlenghtofline(&c2, &c3),
        perpendicular: findlenghtofline(&c1, &c3),
    };
    println!("{:?}", rt);
    println!("{:#?}", rt);
    let cos = cosine(&rt);
    println!("{}", cos);
    println!("{}", rt.cos());
}

struct User {
    username: String,
    age: i32,
    active_user: bool,
}

fn make_user(username: String, age: i32) -> User {
    let active_user = true;
    User {
        username,
        age,
        active_user,
    }
}

struct Cordinates(i32, i32, i32);

#[derive(Debug)]
struct RightTriangle {
    hypotenuse: i32,
    perpendicular: i32,
    base: i32,
}
impl RightTriangle {
    fn cos(&self) -> i32 {
        cosine(self)
    }
}

fn calculateareaoftriangle(c1: &Cordinates, c2: &Cordinates, c3: &Cordinates) -> i32 {
    (c1.0 * (c2.1 - c3.1) + c2.0 * (c1.1 - c3.1) + c3.0 * (c2.1 - c1.1)) / 2
}
fn findlenghtofline(c1: &Cordinates, c2: &Cordinates) -> i32 {
    (c2.0 - c1.0) * (c2.0 - c1.0) + (c2.1 - c1.1) * (c2.1 - c1.1)
}
fn cosine(rt: &RightTriangle) -> i32 {
    rt.perpendicular / rt.hypotenuse
}
