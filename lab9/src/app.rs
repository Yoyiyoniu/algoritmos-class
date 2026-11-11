use crate::estudiante::{Algoritmo, Estudiante, Filtros, Registro};
use crate::vista_alta;
use crate::vista_buscar;
use crate::vista_listado;
use crate::vista_modificar;
use eframe::egui;

pub(crate) const VIEWS: [&str; 4] = ["➕ Alta", "✏ Modificar", "🔍 Buscar", "📋 Listado"];

pub(crate) const EXIT_BUTTON_SIZE: [f32; 2] = [90.0, 30.0];
pub(crate) const SUBMIT_BUTTON_SIZE: [f32; 2] = [160.0, 32.0];
pub(crate) const SPACE_SM: f32 = 6.0;
pub(crate) const SPACE_MD: f32 = 8.0;
pub(crate) const SPACE_LG: f32 = 12.0;

pub(crate) const NARANJA: egui::Color32 = egui::Color32::from_rgb(255, 109, 0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CriterioBusqueda {
    Nombre,
    Matricula,
}

pub(crate) struct EstudiantesApp {
    pub(crate) registro: Registro,
    pub(crate) view: usize,

    // Alta / modificación
    pub(crate) nombre: String,
    pub(crate) matricula: String,
    pub(crate) grado: String,
    pub(crate) promedio: String,

    // Búsqueda
    pub(crate) criterio: CriterioBusqueda,
    pub(crate) algoritmo: Algoritmo,
    pub(crate) consulta: String,
    pub(crate) ficha: String,

    // Listado con filtros
    pub(crate) filtro_grado_activo: bool,
    pub(crate) filtro_grado: String,
    pub(crate) filtro_rango_activo: bool,
    pub(crate) rango_min: String,
    pub(crate) rango_max: String,
    pub(crate) filtros_aplicados: Option<Filtros>,

    pub(crate) resultado: String,
    pub(crate) error: String,
    pub(crate) is_exiting: bool,
}

impl Default for EstudiantesApp {
    fn default() -> Self {
        Self {
            registro: Registro::con_datos_iniciales(),
            view: 0,
            nombre: String::new(),
            matricula: String::new(),
            grado: String::new(),
            promedio: String::new(),
            criterio: CriterioBusqueda::Matricula,
            algoritmo: Algoritmo::Interpolada,
            consulta: String::new(),
            ficha: String::new(),
            filtro_grado_activo: false,
            filtro_grado: String::new(),
            filtro_rango_activo: false,
            rango_min: "10000".to_owned(),
            rango_max: "99999".to_owned(),
            filtros_aplicados: Some((None, None)),
            resultado: String::new(),
            error: String::new(),
            is_exiting: false,
        }
    }
}

impl EstudiantesApp {
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
        self.matricula.clear();
        self.grado.clear();
        self.promedio.clear();
    }

    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (index, name) in VIEWS.iter().enumerate() {
                if ui.selectable_value(&mut self.view, index, *name).clicked() {
                    self.error.clear();
                    self.resultado.clear();
                    self.ficha.clear();
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

pub(crate) fn ficha(e: &Estudiante) -> String {
    format!(
        "Nombre: {}\nMatrícula: {}\nGrado escolar: {}\nPromedio: {:.1}",
        e.nombre, e.matricula, e.grado, e.promedio
    )
}

impl eframe::App for EstudiantesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sistema de Gestión de Estudiantes - Búsqueda Binaria e Interpolada - UABC");
            ui.add_space(8.0);
            self.show_tabs(ui);
            ui.add_space(SPACE_LG);

            match self.view {
                0 => vista_alta::show(self, ui),
                1 => vista_modificar::show(self, ui),
                2 => vista_buscar::show(self, ui),
                _ => vista_listado::show(self, ui),
            }

            if self.is_exiting {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
