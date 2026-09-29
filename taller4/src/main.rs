use chrono::{Duration, NaiveTime};
use rand::RngExt;
use std::collections::VecDeque;

#[derive(Clone)]
struct Paciente {
    id: i64,
    is_med: bool,
    llegada: i64,
    inicio_consulta: i64,
    fin_consulta: i64,
    inicio_farmacia: i64,
    salida: i64,
}

fn main() {
    let mut recepcion: VecDeque<Paciente> = VecDeque::new();
    let mut farmacia: VecDeque<Paciente> = VecDeque::new();

    let mut consultorio: Option<Paciente> = None;
    let mut botica: Option<Paciente> = None;

    let mut terminados: Vec<Paciente> = Vec::new();

    let tiempo_atencion: i64 = 10 * 60; // 17:00

    let mut time: i64 = 0; // 7:00

    let mut rng = rand::rng();

    let mut ids: i64 = 0;
    let mut next_arrival: i64 = 0;
    let mut rechazados: i64 = 0;
    let mut ultima_tarjeta: i64 = 0;

    while time < tiempo_atencion
        || !recepcion.is_empty()
        || !farmacia.is_empty()
        || consultorio.is_some()
        || botica.is_some()
    {
        // Llegada de pacientes
        if time == next_arrival {
            if time < tiempo_atencion {
                let res_med_prob = rng.random_range(0..100);

                let pac = Paciente {
                    id: ids,
                    is_med: res_med_prob < 75,
                    llegada: time,
                    inicio_consulta: 0,
                    fin_consulta: 0,
                    inicio_farmacia: 0,
                    salida: 0,
                };

                recepcion.push_back(pac);
                ultima_tarjeta = time;
                ids = ids + 1;
            } else {
                // Ya paso la hora de cierre, no se recibe la tarjeta
                rechazados += 1;
            }

            next_arrival = time + rng.random_range(3..=5);
        }

        // Termina la consulta
        if consultorio
            .as_ref()
            .map_or(false, |p| p.fin_consulta == time)
        {
            let mut pac = consultorio.take().unwrap();

            if pac.is_med {
                farmacia.push_back(pac);
            } else {
                pac.salida = time;
                terminados.push(pac);
            }
        }

        // Termina la farmacia
        if botica.as_ref().map_or(false, |p| p.salida == time) {
            let pac = botica.take().unwrap();
            terminados.push(pac);
        }

        // Entra el siguiente a consulta
        if consultorio.is_none() {
            if let Some(mut pac) = recepcion.pop_front() {
                pac.inicio_consulta = time;
                pac.fin_consulta = time + rng.random_range(10..=15);
                consultorio = Some(pac);
            }
        }

        // Entra el siguiente a farmacia
        if botica.is_none() {
            if let Some(mut pac) = farmacia.pop_front() {
                pac.inicio_farmacia = time;
                pac.salida = time + rng.random_range(12..=13);
                botica = Some(pac);
            }
        }

        time = time + 1;
    }

    // Metricas
    let consultados = terminados.len() as i64;
    let recetas = terminados.iter().filter(|p| p.is_med).count() as i64;

    let mut suma_espera_consulta = 0;
    let mut suma_espera_farmacia = 0;
    let mut suma_atencion = 0;

    for pac in &terminados {
        suma_espera_consulta += pac.inicio_consulta - pac.llegada;
        suma_atencion += pac.fin_consulta - pac.inicio_consulta;

        if pac.is_med {
            suma_espera_farmacia += pac.inicio_farmacia - pac.fin_consulta;
            suma_atencion += pac.salida - pac.inicio_farmacia;
        }
    }

    println!("Pacientes consultados -> {}", consultados);
    println!("Pacientes con receta -> {}", recetas);
    println!(
        "Promedio espera cola consulta -> {:.2} min",
        promedio(suma_espera_consulta, consultados)
    );
    println!(
        "Promedio espera cola farmacia -> {:.2} min",
        promedio(suma_espera_farmacia, recetas)
    );
    println!(
        "Promedio atencion (consulta + farmacia) -> {:.2} min",
        promedio(suma_atencion, consultados)
    );
    println!("Ultima tarjeta recibida -> {}", hora(ultima_tarjeta));
    println!(
        "Pacientes sin atender (tarjeta no recibida) -> {}",
        rechazados
    );

    if let Some(ultimo) = terminados.last() {
        let mut espera = ultimo.inicio_consulta - ultimo.llegada;
        if ultimo.is_med {
            espera += ultimo.inicio_farmacia - ultimo.fin_consulta;
        }

        println!(
            "Hora en que se retiro el ultimo paciente -> {}",
            hora(ultimo.salida)
        );
        println!(
            "Tiempo de espera del ultimo paciente (Id {}) -> {} min",
            ultimo.id, espera
        );
    }
}

fn hora(min: i64) -> String {
    let inicio = NaiveTime::from_hms_opt(7, 0, 0).unwrap(); // 7:00
    let h = inicio + Duration::minutes(min);
    h.format("%H:%M").to_string()
}

fn promedio(suma: i64, n: i64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    suma as f64 / n as f64
}
