use crate::list::Tree;
use chrono::{Datelike, Local, NaiveDate, NaiveTime};

const MESES: [&str; 12] = [
    "ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic",
];
const HORA_BASE: u32 = 8;
const MINUTO_BASE: u32 = 15;
const DIAS_ATRAS: i64 = 15;

#[derive(Clone, Debug)]
pub struct Correo {
    pub remitente_original: String,
    pub remitente_mostrado: String,
    pub asunto: String,
    pub fecha: NaiveDate,
    pub hora: NaiveTime,
    pub contador: usize,
    pub cuerpos: Vec<String>,
    pub contestado_por_usuario: bool,
}

impl Correo {
    pub fn new(
        remitente: &str,
        asunto: &str,
        fecha: NaiveDate,
        hora: NaiveTime,
        cuerpo: &str,
    ) -> Self {
        Self {
            remitente_original: remitente.to_string(),
            remitente_mostrado: remitente.to_string(),
            asunto: asunto.to_string(),
            fecha,
            hora,
            contador: 1,
            cuerpos: vec![format!("{}: {}", remitente, cuerpo)],
            contestado_por_usuario: false,
        }
    }

    pub fn contestar_por_usuario(&mut self, respuesta: &str) {
        self.contador += 1;
        if !self.contestado_por_usuario {
            self.remitente_mostrado.push_str(", yo");
            self.contestado_por_usuario = true;
        }
        self.cuerpos.push(format!("Yo: {}", respuesta));
    }

    pub fn responder_por_remitente(&mut self, respuesta: &str) {
        self.contador += 1;
        self.cuerpos
            .push(format!("{}: {}", self.remitente_original, respuesta));
        let ahora = Local::now();
        self.fecha = ahora.date_naive();
        self.hora = ahora.time();
    }

    pub fn encabezado_lista(&self) -> String {
        let hoy = Local::now().date_naive();
        let fecha_str = if self.fecha == hoy {
            self.hora.format("%H:%M").to_string()
        } else {
            let mes_idx = (self.fecha.month() as usize).saturating_sub(1);
            format!(
                "{} {}",
                self.fecha.day(),
                MESES.get(mes_idx).copied().unwrap_or_default()
            )
        };

        let contador_str = if self.contador > 1 {
            format!(" ({})", self.contador)
        } else {
            String::new()
        };

        format!(
            "{} {} - {} [{}]",
            self.remitente_mostrado, contador_str, self.asunto, fecha_str
        )
    }
}

pub struct CorreoManager {
    pub lista: Tree<Correo>,
}

impl CorreoManager {
    pub fn new() -> Self {
        let mut manager = Self { lista: Tree::new() };
        manager.inicializar_15_correos();
        manager
    }

    fn inicializar_15_correos(&mut self) {
        let remitentes = [
            "Aquiles Baeza",
            "Elsa Pato",
            "Susana Oria",
            "Aitor Tilla",
            "Debora Melo",
            "Armando Casas",
            "Elba Lazo",
            "Benito Camelo",
            "Zoila Vaca",
            "Lola Mento",
            "Esteban Dido",
            "Elba Surita",
            "Marcia Ana",
            "Inés Tornillo",
            "Zacarías Flores",
        ];

        let hoy = Local::now().date_naive();

        for (i, remitente) in remitentes.iter().enumerate() {
            let fecha = hoy - chrono::Duration::days(DIAS_ATRAS - i as i64);
            let hora = NaiveTime::from_hms_opt(
                HORA_BASE + (i % 10) as u32,
                (MINUTO_BASE + i as u32 * 2) % 60,
                0,
            )
            .unwrap_or_default();

            let correo = Correo::new(
                remitente,
                &format!("Asunto sobre tema {}", i + 1),
                fecha,
                hora,
                &format!("Texto de mensaje inicial del correo {}", i + 1),
            );
            self.lista.agregar_inicio(correo);
        }
    }
}
