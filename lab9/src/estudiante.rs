use std::cmp::Ordering;

pub(crate) const MAX_ESTUDIANTES: usize = 100;
pub(crate) const GRADO_MIN: u8 = 1;
pub(crate) const GRADO_MAX: u8 = 6;
pub(crate) const MATRICULA_MIN: u32 = 10000;
pub(crate) const MATRICULA_MAX: u32 = 99999;

#[derive(Clone, Debug)]
pub(crate) struct Estudiante {
    pub(crate) nombre: String,
    pub(crate) matricula: u32,
    pub(crate) grado: u8,
    pub(crate) promedio: f64,
}

impl Estudiante {
    fn clave_nombre(&self) -> String {
        normalizar(&self.nombre)
    }
}

pub(crate) fn normalizar(texto: &str) -> String {
    texto.trim().to_lowercase()
}

/// Resultado de una búsqueda: posición (si existe) y pasos realizados.
/// Filtro de grado y rango de matrícula (inclusivo); `None` = filtro apagado.
pub(crate) type Filtros = (Option<u8>, Option<(u32, u32)>);

pub(crate) struct Busqueda {
    pub(crate) indice: Option<usize>,
    pub(crate) pasos: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Algoritmo {
    Lineal,
    Binaria,
    Interpolada,
}

impl Algoritmo {
    pub(crate) fn nombre(self) -> &'static str {
        match self {
            Algoritmo::Lineal => "Lineal",
            Algoritmo::Binaria => "Binaria",
            Algoritmo::Interpolada => "Interpolada",
        }
    }
}

/// Almacenamiento en arreglo (máximo 100), ordenado por matrícula.
/// `por_nombre` guarda posiciones ordenadas por nombre para la búsqueda binaria.
pub(crate) struct Registro {
    estudiantes: Vec<Estudiante>,
    por_nombre: Vec<usize>,
}

impl Registro {
    pub(crate) fn new() -> Self {
        Self {
            estudiantes: Vec::with_capacity(MAX_ESTUDIANTES),
            por_nombre: Vec::with_capacity(MAX_ESTUDIANTES),
        }
    }

    pub(crate) fn con_datos_iniciales() -> Self {
        let mut registro = Self::new();
        for (nombre, matricula, grado, promedio) in DATOS_INICIALES {
            let _ = registro.insertar(Estudiante {
                nombre: (*nombre).to_owned(),
                matricula: *matricula,
                grado: *grado,
                promedio: *promedio,
            });
        }
        registro
    }

    pub(crate) fn total(&self) -> usize {
        self.estudiantes.len()
    }

    #[cfg(test)]
    pub(crate) fn lista(&self) -> &[Estudiante] {
        &self.estudiantes
    }

    pub(crate) fn obtener(&self, indice: usize) -> &Estudiante {
        &self.estudiantes[indice]
    }

    pub(crate) fn existe_matricula(&self, matricula: u32) -> bool {
        self.buscar_interpolada(matricula).indice.is_some()
    }

    /// Inserta manteniendo el orden por matrícula.
    pub(crate) fn insertar(&mut self, estudiante: Estudiante) -> Result<(), String> {
        if self.estudiantes.len() >= MAX_ESTUDIANTES {
            return Err(format!(
                "El arreglo está lleno (máximo {MAX_ESTUDIANTES} estudiantes)."
            ));
        }
        if self.existe_matricula(estudiante.matricula) {
            return Err(format!(
                "Matrícula duplicada: {} ya está registrada.",
                estudiante.matricula
            ));
        }
        let posicion = self
            .estudiantes
            .partition_point(|e| e.matricula < estudiante.matricula);
        self.estudiantes.insert(posicion, estudiante);
        self.reconstruir_indice();
        Ok(())
    }

    /// Modifica los datos indicados de la posición dada (la matrícula no cambia).
    pub(crate) fn modificar(
        &mut self,
        indice: usize,
        nombre: Option<String>,
        grado: Option<u8>,
        promedio: Option<f64>,
    ) {
        let e = &mut self.estudiantes[indice];
        if let Some(n) = nombre {
            e.nombre = n;
        }
        if let Some(g) = grado {
            e.grado = g;
        }
        if let Some(p) = promedio {
            e.promedio = p;
        }
        self.reconstruir_indice();
    }

    fn reconstruir_indice(&mut self) {
        self.por_nombre = (0..self.estudiantes.len()).collect();
        let est = &self.estudiantes;
        self.por_nombre.sort_by_key(|&i| est[i].clave_nombre());
    }

    pub(crate) fn buscar_lineal_matricula(&self, matricula: u32) -> Busqueda {
        let mut pasos = 0;
        for (i, e) in self.estudiantes.iter().enumerate() {
            pasos += 1;
            if e.matricula == matricula {
                return Busqueda {
                    indice: Some(i),
                    pasos,
                };
            }
        }
        Busqueda {
            indice: None,
            pasos,
        }
    }

    pub(crate) fn buscar_lineal_nombre(&self, nombre: &str) -> Busqueda {
        let clave = normalizar(nombre);
        let mut pasos = 0;
        for (i, e) in self.estudiantes.iter().enumerate() {
            pasos += 1;
            if e.clave_nombre() == clave {
                return Busqueda {
                    indice: Some(i),
                    pasos,
                };
            }
        }
        Busqueda {
            indice: None,
            pasos,
        }
    }

    /// Búsqueda binaria por nombre (sin distinguir mayúsculas).
    pub(crate) fn buscar_binaria_nombre(&self, nombre: &str) -> Busqueda {
        let clave = normalizar(nombre);
        let mut pasos = 0;
        let (mut bajo, mut alto) = (0isize, self.por_nombre.len() as isize - 1);
        while bajo <= alto {
            pasos += 1;
            let medio = (bajo + alto) / 2;
            let pos = self.por_nombre[medio as usize];
            match self.estudiantes[pos].clave_nombre().cmp(&clave) {
                Ordering::Equal => {
                    return Busqueda {
                        indice: Some(pos),
                        pasos,
                    };
                }
                Ordering::Less => bajo = medio + 1,
                Ordering::Greater => alto = medio - 1,
            }
        }
        Busqueda {
            indice: None,
            pasos,
        }
    }

    /// Búsqueda binaria por matrícula.
    pub(crate) fn buscar_binaria_matricula(&self, matricula: u32) -> Busqueda {
        let mut pasos = 0;
        let (mut bajo, mut alto) = (0isize, self.estudiantes.len() as isize - 1);
        while bajo <= alto {
            pasos += 1;
            let medio = (bajo + alto) / 2;
            match self.estudiantes[medio as usize].matricula.cmp(&matricula) {
                Ordering::Equal => {
                    return Busqueda {
                        indice: Some(medio as usize),
                        pasos,
                    };
                }
                Ordering::Less => bajo = medio + 1,
                Ordering::Greater => alto = medio - 1,
            }
        }
        Busqueda {
            indice: None,
            pasos,
        }
    }

    /// Búsqueda interpolada por matrícula.
    pub(crate) fn buscar_interpolada(&self, matricula: u32) -> Busqueda {
        let v = &self.estudiantes;
        let mut pasos = 0;
        if v.is_empty() {
            return Busqueda {
                indice: None,
                pasos,
            };
        }
        let (mut bajo, mut alto) = (0usize, v.len() - 1);
        let clave = i64::from(matricula);

        while bajo <= alto && matricula >= v[bajo].matricula && matricula <= v[alto].matricula {
            pasos += 1;
            let (vb, va) = (i64::from(v[bajo].matricula), i64::from(v[alto].matricula));
            if vb == va {
                let indice = (v[bajo].matricula == matricula).then_some(bajo);
                return Busqueda { indice, pasos };
            }
            let pos = bajo + ((clave - vb) * (alto - bajo) as i64 / (va - vb)) as usize;
            match v[pos].matricula.cmp(&matricula) {
                Ordering::Equal => {
                    return Busqueda {
                        indice: Some(pos),
                        pasos,
                    };
                }
                Ordering::Less => bajo = pos + 1,
                Ordering::Greater => {
                    if pos == 0 {
                        break;
                    }
                    alto = pos - 1;
                }
            }
        }
        Busqueda {
            indice: None,
            pasos,
        }
    }

    pub(crate) fn buscar_matricula(&self, matricula: u32, algoritmo: Algoritmo) -> Busqueda {
        match algoritmo {
            Algoritmo::Lineal => self.buscar_lineal_matricula(matricula),
            Algoritmo::Binaria => self.buscar_binaria_matricula(matricula),
            Algoritmo::Interpolada => self.buscar_interpolada(matricula),
        }
    }

    pub(crate) fn buscar_nombre(&self, nombre: &str, algoritmo: Algoritmo) -> Busqueda {
        match algoritmo {
            Algoritmo::Lineal => self.buscar_lineal_nombre(nombre),
            // La interpolación solo aplica a claves numéricas: se usa binaria.
            Algoritmo::Binaria | Algoritmo::Interpolada => self.buscar_binaria_nombre(nombre),
        }
    }

    pub(crate) fn filtrar(&self, (grado, rango): Filtros) -> Vec<&Estudiante> {
        self.estudiantes
            .iter()
            .filter(|e| grado.is_none_or(|g| e.grado == g))
            .filter(|e| rango.is_none_or(|(min, max)| e.matricula >= min && e.matricula <= max))
            .collect()
    }
}

const DATOS_INICIALES: &[(&str, u32, u8, f64)] = &[
    ("Sofía Ramírez Gómez", 10245, 4, 92.5),
    ("Alejandro Torres Luna", 34781, 6, 88.9),
    ("Valeria Castro Díaz", 51903, 2, 75.2),
    ("Daniel Morales Ruiz", 28367, 5, 95.0),
    ("Isabella Pérez Soto", 40112, 1, 68.7),
    ("Santiago Vargas Gil", 15639, 3, 81.3),
    ("Camila Flores Nieto", 39084, 6, 99.1),
    ("Mateo Jiménez Vega", 55021, 4, 72.8),
    ("Mariana Reyes Cruz", 21456, 2, 84.6),
    ("Lucas Hernández Paz", 48790, 5, 65.1),
    ("Elena Sánchez Solís", 11973, 1, 90.4),
    ("Sebastián Bravo Gil", 33528, 3, 78.0),
    ("Paula Acosta Rivas", 50164, 6, 86.5),
    ("Emilio Navarro Díaz", 26805, 4, 93.8),
    ("Regina Mendoza López", 42317, 2, 55.9),
    ("Nicolás Ferrer Mar", 18940, 5, 80.2),
    ("Frida Castro Moya", 37691, 1, 97.5),
    ("Alonso Peña Soto", 53419, 3, 71.0),
    ("Renata Blanco Victoria", 24082, 6, 89.4),
    ("Juan Pablo Arias Pérez", 46255, 4, 62.3),
    ("Ximena Ortiz Lee", 13706, 2, 85.7),
    ("Arturo Ramos Rey", 31837, 5, 94.1),
    ("Victoria Soto Paz", 57188, 1, 79.9),
    ("Benjamín Lira Victoria", 20592, 3, 91.6),
    ("Andrea Pardo Gil", 44973, 6, 77.3),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busquedas_coinciden() {
        let r = Registro::con_datos_iniciales();
        assert_eq!(r.total(), 25);
        for e in r.lista() {
            for alg in [
                Algoritmo::Lineal,
                Algoritmo::Binaria,
                Algoritmo::Interpolada,
            ] {
                let i = r.buscar_matricula(e.matricula, alg).indice.unwrap();
                assert_eq!(r.obtener(i).matricula, e.matricula);
            }
            let i = r
                .buscar_binaria_nombre(&e.nombre.to_uppercase())
                .indice
                .unwrap();
            assert_eq!(r.obtener(i).matricula, e.matricula);
        }
        assert!(r.buscar_interpolada(15666).indice.is_none());
        assert!(r.buscar_interpolada(1).indice.is_none());
        assert!(r.buscar_interpolada(99999).indice.is_none());
        assert!(
            r.buscar_binaria_nombre("Francisco Pérez Tejada")
                .indice
                .is_none()
        );
    }

    #[test]
    fn filtros_del_pdf() {
        let r = Registro::con_datos_iniciales();
        assert_eq!(r.filtrar((Some(2), None)).len(), 4);
        assert_eq!(r.filtrar((None, Some((25000, 49999)))).len(), 12);
        // El PDF dice 4, pero con sus datos solo hay 3 (34781, 39084, 44973).
        assert_eq!(r.filtrar((Some(6), Some((25000, 49999)))).len(), 3);
        assert_eq!(r.filtrar((None, None)).len(), 25);
    }

    #[test]
    fn duplicada_y_modificar() {
        let mut r = Registro::con_datos_iniciales();
        let dup = Estudiante {
            nombre: "X Y".into(),
            matricula: 11973,
            grado: 1,
            promedio: 80.0,
        };
        assert!(r.insertar(dup).is_err());
        let i = r.buscar_interpolada(15639).indice.unwrap();
        r.modificar(i, Some("Aaron Zeta".into()), None, None);
        assert!(r.buscar_binaria_nombre("aaron zeta").indice.is_some());
        assert!(
            r.buscar_binaria_nombre("Santiago Vargas Gil")
                .indice
                .is_none()
        );
    }
}
