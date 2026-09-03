use std::fs;

fn main() {
    let ruta = "/home/yoyoz/dev/algoritmos-class/lab2/src/data.txt";
    let contenido = fs::read_to_string(ruta).expect("No se pudo leer el archivo");

    let parr: Vec<&str> = contenido
        .split('.')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    procesar_parrafos(&parr, 1);
}

fn procesar_parrafos(parrafos: &[&str], idx: usize) {
    if parrafos.is_empty() {
        return;
    }

    let lineas: Vec<&str> = parrafos[0].lines().collect();

    if idx & 1 == 0 {
        procesar_par(&lineas, 0);
    } else {
        procesar_impar(&lineas, lineas.len());
    }

    print!("\n");
    procesar_parrafos(&parrafos[1..], idx + 1);
}

fn procesar_impar(lineas: &[&str], i: usize) {
    if i == 0 {
        return;
    }
    let palabras: Vec<&str> = lineas[i - 1].split_whitespace().collect();
    println!("{}.", invertir_orden_palabras(&palabras));
    procesar_impar(lineas, i - 1);
}

fn procesar_par(lineas: &[&str], i: usize) {
    if i >= lineas.len() {
        return;
    }
    let palabras: Vec<&str> = lineas[i].split_whitespace().collect();
    println!("{}.", invertir_letras_palabras(&palabras, 0));
    procesar_par(lineas, i + 1);
}

fn invertir_orden_palabras(palabras: &[&str]) -> String {
    if palabras.len() <= 1 {
        return palabras.get(0).unwrap_or(&"").to_string();
    }
    let ultima = palabras[palabras.len() - 1];
    let resto = invertir_orden_palabras(&palabras[..palabras.len() - 1]);
    format!("{} {}", ultima, resto)
}

fn invertir_letras_palabras(palabras: &[&str], i: usize) -> String {
    if i >= palabras.len() {
        return String::new();
    }
    let invertida = invertir_letras(palabras[i]);
    if i == palabras.len() - 1 {
        invertida
    } else {
        format!(
            "{} {}",
            invertida,
            invertir_letras_palabras(palabras, i + 1)
        )
    }
}

fn invertir_letras(palabra: &str) -> String {
    invertir_chars(&palabra.chars().collect::<Vec<char>>())
}

fn invertir_chars(chars: &[char]) -> String {
    if chars.is_empty() {
        return String::new();
    }
    let mut resto = invertir_chars(&chars[1..]);
    resto.push(chars[0]);
    resto
}
