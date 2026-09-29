mod stack;
use stack::Stack;

fn main() {
    let casos = [
        "A + B * C + D",
        "(A + B) * (C + D)",
        "A * B + C * D",
        "A + B + C + D",
        "(A + B) * C - (D - E) * (F + G)",
    ];

    for exp in casos {
        println!("Infija: {}", exp);
        print!("Prefija: ");
        prefija(exp);
        print!("Postfija: ");
        postfija(exp);
        println!();
    }
}

fn prefija(exp: &str) {
    let mut pila: Stack<char> = Stack::new();
    let mut resultado: Stack<char> = Stack::new();

    for i in exp.chars().rev() {
        match i {
            ' ' => {}
            ')' => pila.push(i),
            '(' => {
                while let Some(op) = pila.pop() {
                    if op == ')' {
                        break;
                    }
                    resultado.push(op);
                }
            }
            '+' | '-' | '*' | '/' => {
                while let Some(&top) = pila.peek() {
                    let prec_tope: i32;
                    if top == '*' || top == '/' {
                        prec_tope = 2;
                    } else {
                        prec_tope = 1;
                    }

                    let prec_actual: i32;
                    if i == '*' || i == '/' {
                        prec_actual = 2;
                    } else {
                        prec_actual = 1;
                    }

                    if prec_tope > prec_actual {
                        resultado.push(pila.pop().unwrap());
                    } else {
                        break;
                    }
                }
                pila.push(i);
            }
            _ => resultado.push(i),
        }
    }

    while let Some(op) = pila.pop() {
        resultado.push(op);
    }

    resultado.reverce();
    resultado.print_all();
}

fn postfija(exp: &str) {
    let mut pila: Stack<char> = Stack::new();
    let mut resultado: Stack<char> = Stack::new();

    for i in exp.chars() {
        match i {
            ' ' => {}
            '(' => pila.push(i),
            ')' => {
                while let Some(op) = pila.pop() {
                    if op == '(' {
                        break;
                    }
                    resultado.push(op);
                }
            }
            '+' | '-' | '*' | '/' => {
                while let Some(&top) = pila.peek() {
                    if top == '(' {
                        break;
                    }

                    let prec_tope: i32;
                    if top == '*' || top == '/' {
                        prec_tope = 2;
                    } else {
                        prec_tope = 1;
                    }

                    let prec_actual: i32;
                    if i == '*' || i == '/' {
                        prec_actual = 2;
                    } else {
                        prec_actual = 1;
                    }

                    if prec_tope >= prec_actual {
                        resultado.push(pila.pop().unwrap());
                    } else {
                        break;
                    }
                }
                pila.push(i);
            }
            _ => resultado.push(i),
        }
    }

    while let Some(op) = pila.pop() {
        resultado.push(op);
    }

    resultado.print_all();
}
