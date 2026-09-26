use rand::RngExt;
use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back("Tastk 1".to_string());
    queue.push_back("Tastk 2".to_string());
    queue.push_back("Tastk 3".to_string());

    println!("{}", queue.back().unwrap());

    let mut rng = rand::rng();

    println!("{}", rng.random_range(0..10));
}
