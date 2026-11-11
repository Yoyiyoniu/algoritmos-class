mod grafo;

use grafo::*;
use std::io::{self, Write};

fn main() {
    let mut g = Grafo::new();
    let mut dinamica = Dinamica::Normal;
    println!("Práctica 8-1 | guía de trayectos con Floyd-Warshall");

    loop {
        println!("\nDinámica actual: {dinamica}");
        println!("1) Viajar (automatizado)");
        println!("2) Viajar (manual)");
        println!("3) Cambiar dinámica de la ciudad");
        println!("4) Ver matrices de distancias y recorridos");
        println!("5) Ver mapa de la ciudad");
        println!("6) Salir");
        match lee("Opción: ").as_str() {
            "1" => viaje(&g, false),
            "2" => viaje(&g, true),
            "3" => {
                dinamica = elige_dinamica();
                g.aplicar(dinamica);
                let (d, r) = floyd(&g.matriz());
                imprime_matrices(&d, &r);
            }
            "4" => {
                let (d, r) = floyd(&g.matriz());
                imprime_matrices(&d, &r);
            }
            "5" => mapa(&g),
            "6" => break,
            _ => println!("Opción inválida."),
        }
    }
}

fn lee(texto: &str) -> String {
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).unwrap_or(0) == 0 {
        println!();
        std::process::exit(0);
    }
    s.trim().to_string()
}

fn lee_lugar(mensaje: &str) -> usize {
    loop {
        println!("{mensaje}");
        for (i, lugar) in LUGARES.iter().enumerate() {
            println!(" {:>2}. {}", i + 1, lugar);
        }
        match lee("> ").parse::<usize>() {
            Ok(n) if (1..=LUGARES.len()).contains(&n) => return n - 1,
            _ => println!("Opción inválida."),
        }
    }
}

fn elige_vecino(pos: usize, m: &[Vec<i64>]) -> usize {
    println!("Lugares conectados desde {}:", LUGARES[pos]);
    let mut opciones = Vec::new();
    for (x, &w) in m[pos].iter().enumerate() {
        if w != INF && x != pos {
            opciones.push((x, w));
        }
    }
    for (i, (x, w)) in opciones.iter().enumerate() {
        println!(" {}. {} ({} min)", i + 1, LUGARES[*x], w);
    }
    loop {
        match lee("> ").parse::<usize>() {
            Ok(n) if (1..=opciones.len()).contains(&n) => return opciones[n - 1].0,
            _ => println!("Opción inválida."),
        }
    }
}

fn viaje(g: &Grafo, manual: bool) {
    let m = g.matriz();
    let (d, r) = floyd(&m);
    let origen = lee_lugar("Punto de partida:");
    let destino = lee_lugar("Punto de llegada:");
    if origen == destino {
        println!("Ya estás en {}.", LUGARES[origen]);
        return;
    }
    if manual {
        viaje_manual(&m, &d, &r, origen, destino);
    } else {
        viaje_auto(&d, &r, origen, destino);
    }
}

fn viaje_auto(d: &[Vec<i64>], r: &[Vec<usize>], origen: usize, destino: usize) {
    if d[origen][destino] == INF {
        println!(
            "No existe ruta entre {} y {}.",
            LUGARES[origen], LUGARES[destino]
        );
        return;
    }
    let mut p = Vec::new();
    ruta(r, origen, destino, &mut p);
    let camino = p
        .iter()
        .map(|&i| LUGARES[i])
        .collect::<Vec<_>>()
        .join(" - ");
    println!("\nCamino: {camino}");
    println!("Distancia: {} min", d[origen][destino]);
}

fn viaje_manual(m: &[Vec<i64>], d: &[Vec<i64>], r: &[Vec<usize>], origen: usize, destino: usize) {
    let mut pos = origen;
    let mut total = 0;
    let mut caminado = vec![origen];
    loop {
        if d[pos][destino] == INF {
            println!(
                "No hay ruta posible desde {} hacia {}.",
                LUGARES[pos], LUGARES[destino]
            );
            println!("El recorrido queda inconcluso.");
            break;
        }
        let mut sugerencia = Vec::new();
        ruta(&r, pos, destino, &mut sugerencia);
        let siguiente = sugerencia[1];
        println!(
            "\nEstás en {}. Te sugiero avanzar hacia {} ({} min). ¿Aceptas? (s/n)",
            LUGARES[pos], LUGARES[siguiente], m[pos][siguiente]
        );

        let acepta = {
            let rpta = lee("> ");
            rpta.eq_ignore_ascii_case("s") || rpta.eq_ignore_ascii_case("si")
        };
        let paso = if acepta {
            siguiente
        } else {
            elige_vecino(pos, &m)
        };

        total += m[pos][paso];
        caminado.push(paso);
        pos = paso;
        if pos == destino {
            break;
        }
    }

    let camino = caminado
        .iter()
        .map(|&i| LUGARES[i])
        .collect::<Vec<_>>()
        .join(" - ");
    println!("\nDe {} a {}.", LUGARES[origen], LUGARES[destino]);
    println!("Trayecto recorrido: {camino}");
    println!("Minutos: {total}");
}

fn elige_dinamica() -> Dinamica {
    loop {
        println!("1) Normal");
        println!("2) Gana las águilas");
        println!("3) Bloqueo");
        println!("4) Empieza a llover");
        match lee("> ").parse::<usize>() {
            Ok(1) => return Dinamica::Normal,
            Ok(2) => return Dinamica::Aguilas,
            Ok(3) => return Dinamica::Bloqueo,
            Ok(4) => return Dinamica::Lluvia,
            _ => println!("Opción inválida."),
        }
    }
}

fn mapa(g: &Grafo) {
    println!("\nMapa de la ciudad (pesos en minutos):");
    for t in g.tramos() {
        if t.solo {
            println!(
                "   {} → {} ({} min) [solo un sentido]",
                LUGARES[t.a], LUGARES[t.b], t.min
            );
        } else {
            println!("   {} ↔ {} ({} min)", LUGARES[t.a], LUGARES[t.b], t.min);
        }
    }
}
