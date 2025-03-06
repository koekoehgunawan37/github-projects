use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("foo", 1);
    map.insert("bar", 2);
    map.insert("baz", 3);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
