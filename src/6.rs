use std::fs;

fn main() {
    let path = "path/to/file";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    println!("{}", contents);
}
