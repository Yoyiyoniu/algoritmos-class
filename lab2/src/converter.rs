use crate::stack::Stack;

const MIN_BASE: u32 = 2;
const MAX_BASE: u32 = 16;

fn char_to_val(digit: char) -> Option<i64> {
    match digit {
        '0'..='9' => Some((digit as u8 - b'0') as i64),
        'A'..='F' | 'a'..='f' => Some((digit.to_ascii_uppercase() as u8 - b'A') as i64 + 10),
        _ => None,
    }
}

fn val_to_char(value: i64) -> char {
    if value < 10 {
        (b'0' + value as u8) as char
    } else {
        (b'A' + (value - 10) as u8) as char
    }
}

fn validate_base(base: u32) -> Result<(), String> {
    if !(MIN_BASE..=MAX_BASE).contains(&base) {
        return Err(format!("Base debe estar entre {MIN_BASE} y {MAX_BASE}"));
    }
    Ok(())
}

fn digit_in_base(digit: char, base: u32) -> Result<i64, String> {
    let value =
        char_to_val(digit).ok_or_else(|| format!("Dígito '{digit}' no válido en base {base}"))?;
    if value >= base as i64 {
        return Err(format!("Dígito '{digit}' no válido en base {base}"));
    }
    Ok(value)
}

fn parse_integer_part(digits: &str, base: u32) -> Result<i64, String> {
    let mut entera: i64 = 0;
    for digit in digits.chars() {
        let value = digit_in_base(digit, base)?;
        entera = entera * base as i64 + value;
    }
    Ok(entera)
}

fn parse_fraction_part(digits: &str, base: u32) -> Result<f64, String> {
    let mut frac: f64 = 0.0;
    let mut divisor = base as f64;
    for digit in digits.chars() {
        let value = digit_in_base(digit, base)?;
        frac += value as f64 / divisor;
        divisor *= base as f64;
    }
    Ok(frac)
}

fn parse_to_decimal(text: &str, base_orig: u32) -> Result<(i64, f64), String> {
    validate_base(base_orig)?;
    let s = text.trim().to_uppercase();
    if s.is_empty() {
        return Err("Ingrese un número".to_string());
    }
    if s.starts_with('-') {
        return Err("No se aceptan numeros negativos".to_string());
    }
    if s.matches('.').count() > 1 {
        return Err("Formato inválido: más de un punto decimal".to_string());
    }

    let (entera_part, frac_part) = match s.split_once('.') {
        Some((a, b)) => (a, b),
        None => (s.as_str(), ""),
    };

    if entera_part.is_empty() && frac_part.is_empty() {
        return Err("Ingrese un número".to_string());
    }

    let entera = parse_integer_part(entera_part, base_orig)?;
    let frac = parse_fraction_part(frac_part, base_orig)?;

    Ok((entera, frac))
}

fn encode_integer_part(entera: i64, base_dest: u32) -> String {
    let divisor = base_dest as i64;
    let mut pila = Stack::new();

    if entera == 0 {
        pila.push(0);
    } else {
        let mut n = entera;
        while n > 0 {
            pila.push(n % divisor);
            n /= divisor;
        }
    }

    let mut entera_str = String::new();
    while let Some(d) = pila.pop() {
        entera_str.push(val_to_char(d));
    }
    entera_str
}

fn encode_fraction_part(frac: f64, base_dest: u32, precision: usize) -> String {
    let mut frac_str = String::new();
    let mut f = frac;
    for _ in 0..precision {
        f *= base_dest as f64;
        let digito = f.trunc() as i64;
        frac_str.push(val_to_char(digito));
        f = f.fract();
    }
    frac_str
}

fn decimal_to_base(entera: i64, frac: f64, base_dest: u32, precision: usize) -> String {
    validate_base(base_dest).expect("base destino ya validada");

    let entera_str = encode_integer_part(entera, base_dest);

    if precision == 0 {
        return entera_str;
    }

    let frac_str = encode_fraction_part(frac, base_dest, precision);
    format!("{entera_str}.{frac_str}")
}

pub fn convert_general(
    numero: &str,
    base_orig: u32,
    base_dest: u32,
    precision: usize,
) -> Result<String, String> {
    validate_base(base_orig)?;
    validate_base(base_dest)?;
    let (entera, frac) = parse_to_decimal(numero, base_orig)?;
    Ok(decimal_to_base(entera, frac, base_dest, precision))
}
