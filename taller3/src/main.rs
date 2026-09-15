mod stack;

use stack::Stack;

fn main() {
    let desi1 = 323.777;
    println!("Convirtiendo el numero {}", desi1);
    println!("Binario: {:?}", convert(desi1, POWERS::BINARY));
    println!("Hexa: {:?}", convert(desi1, POWERS::HEXADECIMAL));
    println!("Octal: {:?}", convert(desi1, POWERS::OCTAL));

    let desi2 = 4787.231;
    println!("Convirtiendo el numero {}", desi2);
    println!("Binario: {:?}", convert(desi2, POWERS::BINARY));
    println!("Hexa: {:?}", convert(desi2, POWERS::HEXADECIMAL));
    println!("Octal: {:?}", convert(desi2, POWERS::OCTAL));

    let desi3 = 299.553;
    println!("Convirtiendo el numero {}", desi3);
    println!("Binario: {:?}", convert(desi3, POWERS::BINARY));
    println!("Hexa: {:?}", convert(desi3, POWERS::HEXADECIMAL));
    println!("Octal: {:?}", convert(desi3, POWERS::OCTAL));
}

#[derive(Clone, Copy)]
enum POWERS {
    BINARY = 2,
    OCTAL = 8,
    HEXADECIMAL = 16,
}

fn convert(num: f64, power: POWERS) -> Result<String, String> {
    if num < 0.0 {
        return Err("No se aceptan numeros negativos".to_string());
    }

    let divisor = power as i64;

    let parte_entera = num.trunc() as i64;
    let mut parte_fraccionaria = num.fract();

    let mut stack = Stack::new();
    let mut entera_digitos = Stack::new();

    let mut n = parte_entera;
    if n == 0 {
        entera_digitos.push(0);
    }
    while n > 0 {
        let digito = n % divisor;
        n = n / divisor;

        stack.push(n);
        entera_digitos.push(digito);
    }

    let mut entera_str = String::new();
    while let Some(d) = entera_digitos.pop() {
        entera_str.push(digito_a_char(d));
    }

    let mut fraccion_str = String::new();
    for _ in 0..5 {
        parte_fraccionaria *= divisor as f64;
        let digito = parte_fraccionaria.trunc() as i64;
        fraccion_str.push(digito_a_char(digito));
        parte_fraccionaria = parte_fraccionaria.fract();
    }

    Ok(format!("{}.{}", entera_str, fraccion_str))
}

fn digito_a_char(d: i64) -> char {
    if d < 10 {
        (b'0' + d as u8) as char
    } else {
        (b'A' + (d - 10) as u8) as char
    }
}
