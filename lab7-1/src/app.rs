use crate::avl::{Arbol, Cliente};
use crate::vista_alta;
use crate::vista_baja;
use crate::vista_correo;
use crate::vista_mostrar;
use eframe::egui;

pub(crate) const VIEWS: [&str; 4] = ["Alta de Cliente", "Mostrar", "Enviar Correo", "Baja"];
pub(crate) const FILTROS: [&str; 4] = [
    "Listado General",
    "Filtrado por Membresía",
    "Individual",
    "Regresar",
];

pub(crate) const EXIT_BUTTON_SIZE: [f32; 2] = [90.0, 30.0];
pub(crate) const SUBMIT_BUTTON_SIZE: [f32; 2] = [160.0, 32.0];
pub(crate) const SPACE_SM: f32 = 6.0;
pub(crate) const SPACE_MD: f32 = 8.0;
pub(crate) const SPACE_LG: f32 = 12.0;

pub(crate) const NARANJA: egui::Color32 = egui::Color32::from_rgb(255, 109, 0);
pub(crate) const ROJO: egui::Color32 = egui::Color32::from_rgb(200, 40, 40);

pub(crate) struct GimnasioApp {
    pub(crate) arbol: Arbol,

    pub(crate) view: usize,
    pub(crate) mostrar_indice: usize,

    pub(crate) nombre: String,
    pub(crate) edad: String,
    pub(crate) telefono: String,
    pub(crate) correo: String,
    pub(crate) atrasadas: String,
    pub(crate) membresia_texto: String,

    pub(crate) filtro_membresia: usize,
    pub(crate) ficha_cliente: String,

    pub(crate) envio_realizado: bool,
    pub(crate) enviada_a: Vec<String>,
    pub(crate) sin_correo: usize,

    pub(crate) resultado: String,
    pub(crate) error: String,
    pub(crate) is_exiting: bool,
}

impl Default for GimnasioApp {
    fn default() -> Self {
        let mut arbol = Arbol::new();
        for cliente in crate::avl::datos_iniciales() {
            arbol.insertar(cliente);
        }

        Self {
            arbol,
            view: 0,
            mostrar_indice: 0,
            nombre: String::new(),
            edad: String::new(),
            telefono: String::new(),
            correo: String::new(),
            atrasadas: "0".to_owned(),
            membresia_texto: "Básica".to_owned(),
            filtro_membresia: 1,
            ficha_cliente: String::new(),
            envio_realizado: false,
            enviada_a: Vec::new(),
            sin_correo: 0,
            resultado: String::new(),
            error: String::new(),
            is_exiting: false,
        }
    }
}

impl GimnasioApp {
    pub(crate) fn set_error(&mut self, mensaje: &str) {
        self.resultado.clear();
        self.error = mensaje.to_owned();
    }

    pub(crate) fn set_result(&mut self, mensaje: &str) {
        self.error.clear();
        self.resultado = mensaje.to_owned();
    }

    pub(crate) fn limpiar_campos(&mut self) {
        self.nombre.clear();
        self.edad.clear();
        self.telefono.clear();
        self.correo.clear();
        self.atrasadas = "0".to_owned();
    }

    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (index, name) in VIEWS.iter().enumerate() {
                if ui.selectable_value(&mut self.view, index, *name).clicked() {
                    self.error.clear();
                    self.resultado.clear();
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    egui::RichText::new("SALIR")
                        .color(egui::Color32::WHITE)
                        .strong(),
                )
                .fill(NARANJA);
                if ui.add_sized(EXIT_BUTTON_SIZE, btn).clicked() {
                    self.is_exiting = true;
                }
            });
        });
    }

    pub(crate) fn mostrar_feedback(&self, ui: &mut egui::Ui) {
        ui.add_space(SPACE_LG);
        if !self.resultado.is_empty() {
            ui.label(egui::RichText::new(&self.resultado).color(egui::Color32::DARK_GREEN));
        }
        if !self.error.is_empty() {
            ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
        }
    }
}

pub(crate) fn ficha(cliente: &Cliente) -> String {
    format!(
        "Nombre: {}\nEdad: {} años\nTeléfono: {}\nCorreo: {}\nMensualidades atrasadas: {}\nMembresía: {}",
        cliente.nombre,
        cliente.edad,
        cliente.telefono,
        cliente
            .correo
            .as_deref()
            .unwrap_or("sin correo electrónico"),
        cliente.mensualidades_atrasadas,
        cliente.membresia.nombre(),
    )
}

impl eframe::App for GimnasioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gimnasio - Base de Clientes (Árbol AVL) - UABC");
            ui.add_space(SPACE_MD);
            self.show_tabs(ui);
            ui.add_space(SPACE_LG);

            match self.view {
                0 => vista_alta::show(self, ui),
                1 => vista_mostrar::show(self, ui),
                2 => vista_correo::show(self, ui),
                3 => vista_baja::show(self, ui),
                _ => vista_alta::show(self, ui),
            }

            if self.is_exiting {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
