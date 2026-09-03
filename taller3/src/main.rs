use stack::Stack;

mod stack;

fn main() {
    let mut stack: Stack<String, 120> = Stack::new();

    stack.push("Pedrito".to_string()).unwrap();
    stack.push("Pablo".to_string()).unwrap();

    stack.pop();

    print!("{:?}", stack.peek());
}
