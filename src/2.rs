use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 4);
    map.insert("cherry", 1);
    println!("{:?}", map);
}
