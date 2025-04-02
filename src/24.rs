use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    map.insert("apple", "fruit");
    map.insert("banana", "fruit");
    map.insert("orange", "fruit");

    println!("{:?}", map);
}
