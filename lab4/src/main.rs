mod queue;

use queue::{ErrorCola, Queue};
use rand::RngExt;
use std::io::{self, Write};

const NUM_LINES: usize = 3;

struct Plant {
    lines: [Queue; NUM_LINES],
    entries: [u32; NUM_LINES],
    next_id: u32,
}

fn main() {
    let mut plant = Plant {
        lines: [Queue::new(), Queue::new(), Queue::new()],
        entries: [0; NUM_LINES],
        next_id: 100,
    };

    loop {
        println!("\nFABRICA DE COMPONENTES");
        println!("1. Ingresar lote");
        println!("2. Despachar lote");
        println!("3. Informe de planta");
        println!("4. Salir");

        match read_number(">>: ") {
            Some(1) => plant.enter_batch(),
            Some(2) => plant.dispatch_batch(),
            Some(3) => plant.report(),
            Some(4) => break,
            _ => eprintln!("[ERROR] Opcion invalida."),
        }
    }

    println!("Sistema apagado.");
}

impl Plant {
    fn enter_batch(&mut self) {
        let id = format!("A-{}", self.next_id);
        self.next_id += 1;
        let mut random = rand::rng();
        let original = random.random_range(0..NUM_LINES);

        println!(
            "\nLote {} - linea seleccionada al azar: {}",
            id,
            original + 1
        );

        for k in 0..NUM_LINES {
            let l = (original + k) % NUM_LINES;
            if self.lines[l].esta_llena() {
                println!("La linea {} esta llena (slots libres: 0).", l + 1);
                continue;
            }

            if let Ok(_) = self.lines[l].encolar(id.clone()) {
                self.entries[l] += 1;
                println!("Lote {} ingresado a la linea {}.", id, l + 1);
                return;
            }
        }

        // Manejo de desalojo forzado cuando todos los buffers fallan
        if let Ok(forced) = self.lines[original].desencolar() {
            println!(
                "[OVERFLOW] Despacho forzado del lote mas antiguo en linea {}: {}",
                original + 1,
                forced
            );
        }

        let _ = self.lines[original].encolar(id.clone());
        self.entries[original] += 1;
        println!("Lote {} ingresado a la linea {}.", id, original + 1);
    }

    fn dispatch_batch(&mut self) {
        let Some(line) = read_number("Numero de linea (1-3): ") else {
            println!("Linea invalida.");
            return;
        };
        if line < 1 || line > NUM_LINES {
            println!("Linea invalida.");
            return;
        };

        match self.lines[line - 1].desencolar() {
            Ok(id) => println!("Lote {} despachado de la linea {}.", id, line),
            Err(ErrorCola::SinElementos) => println!("La linea {} no tiene lotes.", line),
            Err(e) => eprintln!("Error al despachar: {}", e),
        }
    }

    fn report(&self) {
        println!("\nINFORME DE PLANTA");

        let mut max = 0;
        let mut min = 0;

        for l in 1..NUM_LINES {
            if self.entries[l] > self.entries[max] {
                max = l;
            }
            if self.entries[l] < self.entries[min] {
                min = l;
            }
        }

        println!(
            "Linea de mayor trafico: {} ({} lotes)",
            max + 1,
            self.entries[max]
        );
        println!(
            "Linea de menor trafico: {} ({} lotes)",
            min + 1,
            self.entries[min]
        );

        println!("\nEstado actual:");
        for l in 0..NUM_LINES {
            let ids = self.lines[l].obtener_identificadores();
            println!("Linea {}: {} lotes [{}]", l + 1, ids.len(), ids.join(", "));
        }
    }
}

fn read_number(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok()?;
    s.trim().parse().ok()
}
