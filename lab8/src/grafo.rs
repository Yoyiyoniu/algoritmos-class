use std::fmt;

pub const INF: i64 = 4000;

// Grafo transcrito arista por arista de la figura de la práctica 8-1:
// flecha en ambos extremos = doble sentido, flecha en un extremo = un sentido.
pub const LUGARES: [&str; 15] = [
    "Casa",
    "Farmacia",
    "Panadería",
    "Banco",
    "Lavandería",
    "Bolo",
    "Supermercado",
    "Barbería",
    "Hamburguesas",
    "Pizzería",
    "Cine",
    "Librería",
    "Cantina",
    "Escuela",
    "Iglesia",
];

const CASA: usize = 0;
const FARMACIA: usize = 1;
const PANADERIA: usize = 2;
const BANCO: usize = 3;
const LAVANDERIA: usize = 4;
const BOLO: usize = 5;
const SUPER: usize = 6;
const BARBERIA: usize = 7;
const HAMBURGUESAS: usize = 8;
const PIZZERIA: usize = 9;
const CINE: usize = 10;
const LIBRERIA: usize = 11;
const CANTINA: usize = 12;
const ESCUELA: usize = 13;
const IGLESIA: usize = 14;

#[derive(Clone, Copy)]
pub struct Tramo {
    pub a: usize,
    pub b: usize,
    pub min: i64,
    pub solo: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Dinamica {
    Normal,
    Aguilas,
    Bloqueo,
    Lluvia,
}

impl fmt::Display for Dinamica {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dinamica::Normal => write!(f, "Normal"),
            Dinamica::Aguilas => write!(
                f,
                "Gana las águilas (calles de un solo sentido pasan a dos sentidos)"
            ),
            Dinamica::Bloqueo => write!(
                f,
                "Bloqueo (las calles de 3, 4 y 5 min triplican su tiempo)"
            ),
            Dinamica::Lluvia => write!(
                f,
                "Lluvia (se pierden las conexiones de la Cantina y el Bolo)"
            ),
        }
    }
}

pub struct Grafo {
    tramos: Vec<Tramo>,
}

// Doble sentido: ambos extremos con flecha en el mapa.
fn doble(a: usize, b: usize, min: i64) -> Tramo {
    Tramo { a, b, min, solo: false }
}

// Un solo sentido: flecha únicamente en `b`.
fn solo(a: usize, b: usize, min: i64) -> Tramo {
    Tramo { a, b, min, solo: true }
}

fn base() -> Vec<Tramo> {
    vec![
        solo(IGLESIA, FARMACIA, 3),
        doble(IGLESIA, CASA, 4),
        doble(IGLESIA, BOLO, 4),
        doble(IGLESIA, CINE, 14),
        doble(FARMACIA, PANADERIA, 8),
        solo(FARMACIA, BANCO, 3),
        doble(FARMACIA, BOLO, 3),
        doble(FARMACIA, BARBERIA, 8),
        doble(PANADERIA, BANCO, 4),
        solo(PANADERIA, LAVANDERIA, 5),
        doble(PANADERIA, SUPER, 6),
        solo(LAVANDERIA, BANCO, 9),
        doble(LAVANDERIA, SUPER, 3),
        doble(LAVANDERIA, HAMBURGUESAS, 9),
        solo(BANCO, SUPER, 6),
        solo(BANCO, BARBERIA, 6),
        doble(BANCO, HAMBURGUESAS, 12),
        solo(CASA, BOLO, 7),
        solo(BOLO, BARBERIA, 5),
        solo(BARBERIA, CASA, 10),
        doble(CASA, PIZZERIA, 3),
        solo(CASA, LIBRERIA, 9),
        solo(PIZZERIA, BARBERIA, 7),
        solo(BARBERIA, HAMBURGUESAS, 4),
        solo(HAMBURGUESAS, SUPER, 2),
        solo(SUPER, ESCUELA, 8),
        solo(HAMBURGUESAS, ESCUELA, 3),
        solo(BARBERIA, CINE, 4),
        doble(PIZZERIA, CINE, 2),
        doble(LIBRERIA, PIZZERIA, 5),
        solo(LIBRERIA, CINE, 11),
        solo(CINE, CANTINA, 3),
        doble(BARBERIA, CANTINA, 5),
        solo(CANTINA, HAMBURGUESAS, 6),
        solo(ESCUELA, BARBERIA, 9),
        doble(LIBRERIA, CANTINA, 10),
        doble(CANTINA, ESCUELA, 4),
    ]
}

impl Grafo {
    pub fn new() -> Self {
        Grafo { tramos: base() }
    }

    pub fn aplicar(&mut self, din: Dinamica) {
        self.tramos = base();
        match din {
            Dinamica::Normal => {}
            Dinamica::Aguilas => {
                for t in &mut self.tramos {
                    t.solo = false;
                }
            }
            Dinamica::Bloqueo => {
                for t in &mut self.tramos {
                    if (3..=5).contains(&t.min) {
                        t.min *= 3;
                    }
                }
            }
            Dinamica::Lluvia => {
                self.tramos
                    .retain(|t| t.a != CANTINA && t.b != CANTINA && t.a != BOLO && t.b != BOLO);
            }
        }
    }

    pub fn matriz(&self) -> Vec<Vec<i64>> {
        let n = LUGARES.len();
        let mut m = vec![vec![INF; n]; n];
        for t in &self.tramos {
            m[t.a][t.b] = t.min;
            if !t.solo {
                m[t.b][t.a] = t.min;
            }
        }
        for i in 0..n {
            m[i][i] = 0;
        }
        m
    }

    pub fn tramos(&self) -> &[Tramo] {
        &self.tramos
    }
}

pub fn floyd(m: &[Vec<i64>]) -> (Vec<Vec<i64>>, Vec<Vec<usize>>) {
    let n = m.len();
    let mut d = m.to_vec();
    let mut r: Vec<Vec<usize>> = Vec::with_capacity(n);
    for _ in 0..n {
        r.push((0..n).collect());
    }
    for k in 0..n {
        for i in 0..n {
            if i == k || d[i][k] == INF {
                continue;
            }
            for j in 0..n {
                if j == k || d[k][j] == INF {
                    continue;
                }
                let suma = d[i][k] + d[k][j];
                if suma < d[i][j] {
                    d[i][j] = suma;
                    r[i][j] = k;
                }
            }
        }
    }
    (d, r)
}

pub fn ruta(r: &[Vec<usize>], i: usize, j: usize, out: &mut Vec<usize>) {
    let k = r[i][j];
    if k == j {
        out.push(i);
        if i != j {
            out.push(j);
        }
        return;
    }
    ruta(r, i, k, out);
    out.pop();
    ruta(r, k, j, out);
}

fn encabezado() {
    print!("{:>13}", "");
    for lugar in LUGARES {
        print!("{lugar:>13}");
    }
    println!();
}

pub fn imprime_matrices(d: &[Vec<i64>], r: &[Vec<usize>]) {
    println!("\nMatriz de distancias D");
    encabezado();
    for i in 0..LUGARES.len() {
        print!("{:>13}", LUGARES[i]);
        for j in 0..LUGARES.len() {
            if d[i][j] == INF {
                print!("{:>13}", "INF");
            } else {
                print!("{:>13}", d[i][j]);
            }
        }
        println!();
    }
    println!("\nMatriz de recorridos R");
    encabezado();
    for i in 0..LUGARES.len() {
        print!("{:>13}", LUGARES[i]);
        for j in 0..LUGARES.len() {
            print!("{:>13}", LUGARES[r[i][j]]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camino(g: &Grafo, a: usize, b: usize) -> (Vec<&'static str>, i64) {
        let (d, r) = floyd(&g.matriz());
        let mut p = Vec::new();
        ruta(&r, a, b, &mut p);
        (p.iter().map(|&i| LUGARES[i]).collect(), d[a][b])
    }

    #[test]
    fn la_figura_tiene_37_calles() {
        assert_eq!(Grafo::new().tramos().len(), 37);
    }

    #[test]
    fn sentidos_de_la_figura() {
        let m = Grafo::new().matriz();
        assert_eq!(m[IGLESIA][FARMACIA], 3);
        assert_eq!(m[FARMACIA][IGLESIA], INF);
        assert_eq!(m[BARBERIA][CASA], 10);
        assert_eq!(m[CASA][BARBERIA], INF);
        assert_eq!(m[BANCO][HAMBURGUESAS], 12);
        assert_eq!(m[HAMBURGUESAS][BANCO], 12);
    }

    #[test]
    fn rutas_basicas() {
        let g = Grafo::new();
        assert_eq!(camino(&g, CASA, CINE).1, 5);
        assert_eq!(camino(&g, CASA, ESCUELA).1, 12);
    }

    #[test]
    fn lluvia_aisla_cantina_y_bolo() {
        let mut g = Grafo::new();
        g.aplicar(Dinamica::Lluvia);
        let (d, _) = floyd(&g.matriz());
        assert_eq!(d[CASA][CANTINA], INF);
        assert_eq!(d[CASA][BOLO], INF);
    }
}
