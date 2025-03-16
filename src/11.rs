// This is an example of a Rust function that returns a random number between 0 and 100 inclusive.
fn get_random_number() -> i32 {
    rand::thread_rng().gen_range(0..=100)
}
