use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("./src/a.txt");
    let f = match f {
        Ok(file) => {
            println!("File found");
            file
        }
        Err(error) => panic!("file not found {:?}", error),
    };
    read_from_file(String::from("./src/a.txt"));
    let f = File::open("a.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("file not found {:?}", error),
    };
}
use std::fs;

fn read_from_file(path: String) {
    let a = fs::read_to_string(path);
    for l in a {
        println!("{}", l)
    }
}
