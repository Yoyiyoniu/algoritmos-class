mod circuito;
use circuito::Circuito;
use std::io::{self, Write};

type Conexion = (usize, usize, usize);
type Plan = Vec<(usize, Vec<String>)>;

fn construir_circuitos() -> Vec<Circuito> {
    let centro = Circuito::new(
        "Centro Historico",
        &[
            "Auditorio",
            "Museo de Arte Moderno/Museo Tamayo",
            "Fuente de las Cibeles",
            "Monumento a la Independencia",
            "Plaza Comercial Reforma 222",
            "Museo de Cera/Museo Ripley",
            "Glorieta de Colon",
            "Hemiciclo a Juarez",
            "Zocalo",
            "Plaza Manuel Tolsa",
            "Museo Franz Mayer",
            "Monumento a la Revolucion",
            "Senado de la Republica",
            "Monumento a la Independencia",
            "Diana Cazadora, Rio de la Plata",
            "Museo de Antropologia",
        ],
    );

    let sur = Circuito::new(
        "Sur Coyoacan",
        &[
            "Fuente de las Cibeles",
            "Angel de la Independencia",
            "WTC/CETRO/Bellini",
            "Centro Comercial Torre Manacar (Insurgentes)",
            "Museo Frida Kahlo (3 Cruces)",
            "Centro Historico de Coyoacan (Tres Cruces)",
            "Centro Historico de Coyoacan",
            "Museo Frida Kahlo",
            "Centro Comercial Torre Manacar (Barranca del Muerto)",
        ],
    );

    let polanco = Circuito::new(
        "Polanco",
        &[
            "Auditorio",
            "Arquimedes/Campos Eliseos",
            "Punto Mexico/Sectur Federal",
            "Carajillo/Masaryk Restaurante",
            "Uno by Real Madrid/Masaryk",
            "Masaryk/Alejandro Dumas",
            "Masaryk y Moliere/Palacio de los Palacios",
            "Antara Fashion Hall",
            "Museo Soumaya, Acuario Inbursa, Museo Jumex",
            "Julio Verne/Polanquito",
        ],
    );

    let basilica = Circuito::new(
        "Basilica",
        &[
            "Zocalo",
            "Plaza Garibaldi",
            "Basilica de Guadalupe",
            "Acuario Michin / Plaza Parque Tepeyac",
        ],
    );

    vec![centro, sur, polanco, basilica]
}

fn conexiones() -> Vec<Conexion> {
    vec![
        (2, 1, 0),  // Cibeles (Centro) & Cibeles (Sur)
        (13, 1, 1), // Monumento a la Independencia (14) & Angel de la Independencia
        (0, 2, 0),  // Auditorio (Centro) & Auditorio (Polanco)
        (8, 3, 0),  // Zocalo (Centro) & Zocalo (Basilica)
    ]
}

// Todas las ubicaciones (circuito, indice) de una estacion
fn ubicaciones(circuitos: &[Circuito], nombre: &str) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (c, circ) in circuitos.iter().enumerate() {
        for i in circ.find_all(nombre) {
            res.push((c, i));
        }
    }
    res
}

fn costo(plan: &Plan) -> usize {
    plan.iter().map(|(_, e)| e.len()).sum()
}

fn buscar_ruta(
    circuitos: &[Circuito],
    conexiones: &[Conexion],
    origen: &str,
    destino: &str,
) -> Option<Plan> {
    let origenes = ubicaciones(circuitos, origen);
    let destinos = ubicaciones(circuitos, destino);
    let mut mejor: Option<Plan> = None;

    for &(oc, oi) in &origenes {
        for &(dc, di) in &destinos {
            let mut candidatos: Vec<Plan> = Vec::new();

            if oc == dc {
                // Mismo circuito, sin cambios
                candidatos.push(vec![(oc, circuitos[oc].route(oi, di))]);
            } else {
                // Un solo cambio, siempre a traves de Centro Historico
                for &(c, k, ik) in conexiones {
                    if oc == 0 && dc == k {
                        candidatos.push(vec![
                            (0, circuitos[0].route(oi, c)),
                            (k, circuitos[k].route(ik, di)),
                        ]);
                    } else if oc == k && dc == 0 {
                        candidatos.push(vec![
                            (k, circuitos[k].route(oi, ik)),
                            (0, circuitos[0].route(c, di)),
                        ]);
                    }
                }
            }

            for cand in candidatos {
                let mejor_es_menor = match &mejor {
                    Some(m) => costo(m) <= costo(&cand),
                    None => false,
                };
                if !mejor_es_menor {
                    mejor = Some(cand);
                }
            }
        }
    }
    mejor
}

fn leer(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let circuitos = construir_circuitos();
    let conexiones = conexiones();

    let origen = leer("Origen:\n  ");
    let destino = leer("Destino:\n  ");

    if ubicaciones(&circuitos, &origen).is_empty() {
        println!("\nNo se encontro la estacion de origen: {}", origen);
        return;
    }
    if ubicaciones(&circuitos, &destino).is_empty() {
        println!("\nNo se encontro la estacion de destino: {}", destino);
        return;
    }

    match buscar_ruta(&circuitos, &conexiones, &origen, &destino) {
        Some(plan) => {
            println!("\nResultado de la consulta:\n");
            for (c, estaciones) in &plan {
                println!("Circuito {}:", circuitos[*c].name);
                for e in estaciones {
                    println!("{}", e);
                }
                println!();
            }
        }
        None => {
            println!("\nNo hay ruta con un solo cambio de circuito entre esas estaciones.");
        }
    }
}
