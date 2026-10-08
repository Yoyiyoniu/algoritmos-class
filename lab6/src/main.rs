mod email;
mod list;

use chrono::Local;
use eframe::egui;
use email::{Correo, CorreoManager};

const WINDOW_SIZE: [f32; 2] = [960.0, 640.0];
const VIEWS: [&str; 2] = ["Bandeja de Entrada", "Recibir Correo"];

const THREAD_MAX_HEIGHT: f32 = 180.0;
const EXIT_BUTTON_SIZE: [f32; 2] = [90.0, 30.0];
const SUBMIT_BUTTON_SIZE: [f32; 2] = [160.0, 32.0];
const SPACE_SM: f32 = 6.0;
const SPACE_MD: f32 = 8.0;
const SPACE_LG: f32 = 12.0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Correos chidos",
        options,
        Box::new(|_cc| Ok(Box::new(CorreoApp::default()))),
    )
}

struct CorreoApp {
    manager: CorreoManager,
    view: usize,

    seleccion: Option<usize>,

    nuevo_remitente: String,
    nuevo_asunto: String,
    nuevo_cuerpo: String,

    texto_respuesta_usuario: String,
    texto_respuesta_remitente: String,

    resultado: String,
    error: String,
    is_exiting: bool,
}

impl Default for CorreoApp {
    fn default() -> Self {
        Self {
            manager: CorreoManager::new(),
            view: 0,
            seleccion: Some(0),
            nuevo_remitente: String::new(),
            nuevo_asunto: String::new(),
            nuevo_cuerpo: String::new(),
            texto_respuesta_usuario: String::new(),
            texto_respuesta_remitente: String::new(),
            resultado: String::new(),
            error: String::new(),
            is_exiting: false,
        }
    }
}

impl CorreoApp {
    fn set_error(&mut self, message: &str) {
        self.resultado.clear();
        self.error = message.to_owned();
    }

    fn set_result(&mut self, message: &str) {
        self.error.clear();
        self.resultado = message.to_owned();
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
                .fill(egui::Color32::from_rgb(255, 109, 0));
                if ui.add_sized(EXIT_BUTTON_SIZE, btn).clicked() {
                    self.is_exiting = true;
                }
            });
        });
    }

    fn show_inbox(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bandeja de Entrada (Listas Doblemente Enlazadas)");
        ui.add_space(SPACE_SM);

        let total = self.manager.lista.size();
        ui.label(format!("Total de correos: {}", total));
        ui.add_space(SPACE_MD);

        // Vista dividida: Lista a la izquierda | Detalle a la derecha
        ui.columns(2, |cols| {
            // Columna Izquierda: todos los correos, con scroll
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Seleccione un correo:").strong());
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("lista_correos")
                    .auto_shrink([false, false])
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        for i in 0..total {
                            let Some(nodo) = self.manager.lista.obtener_nodo_por_indice(i) else {
                                continue;
                            };
                            let correo = nodo.borrow().dato.clone();
                            let label = format!("[{}] {}", i + 1, correo.encabezado_lista());

                            let is_selected = self.seleccion == Some(i);
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.seleccion = Some(i);
                            }
                        }
                    });
            });

            // Columna Derecha: Hilo de conversación y acciones de respuesta
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Hilo de Conversación:").strong());
                ui.separator();

                if let Some(abs_idx) = self.seleccion {
                    let Some(nodo) = self.manager.lista.obtener_nodo_por_indice(abs_idx) else {
                        ui.label("Ningún correo seleccionado.");
                        return;
                    };
                    let correo = nodo.borrow().dato.clone();

                    ui.label(format!("Asunto: {}", correo.asunto));
                    ui.label(format!("Remitente: {}", correo.remitente_mostrado));
                    ui.label(format!("Mensajes: {}", correo.contador));
                    ui.add_space(SPACE_SM);

                    egui::ScrollArea::vertical()
                        .id_salt("hilo_conversacion")
                        .max_height(THREAD_MAX_HEIGHT)
                        .show(ui, |ui| {
                            for cuerpo in &correo.cuerpos {
                                ui.label(cuerpo);
                                ui.separator();
                            }
                        });

                    ui.add_space(SPACE_MD);
                    ui.label(egui::RichText::new("Contestar como Usuario (Yo):").small());
                    ui.text_edit_singleline(&mut self.texto_respuesta_usuario);
                    if ui.button("Enviar Respuesta (Yo)").clicked()
                        && !self.texto_respuesta_usuario.trim().is_empty()
                    {
                        let respuesta = std::mem::take(&mut self.texto_respuesta_usuario);
                        nodo.borrow_mut()
                            .dato
                            .contestar_por_usuario(respuesta.trim());
                        self.set_result(
                            "Respuesta enviada. Se agregó ', yo' y se incrementó el contador.",
                        );
                    }

                    ui.add_space(SPACE_MD);
                    ui.label(
                        egui::RichText::new("Simular respuesta de Remitente Original:").small(),
                    );
                    ui.text_edit_singleline(&mut self.texto_respuesta_remitente);
                    if ui.button("Recibir Respuesta de Remitente").clicked()
                        && !self.texto_respuesta_remitente.trim().is_empty()
                    {
                        let respuesta = std::mem::take(&mut self.texto_respuesta_remitente);
                        nodo.borrow_mut()
                            .dato
                            .responder_por_remitente(respuesta.trim());
                        self.manager.lista.mover_al_inicio(abs_idx);
                        self.seleccion = Some(0);
                        self.set_result(
                            "Respuesta recibida. El correo se movió al INICIO de la lista.",
                        );
                    }
                } else {
                    ui.label("Ningún correo seleccionado.");
                }
            });
        });

        self.show_feedback(ui);
    }

    fn show_new_mail(&mut self, ui: &mut egui::Ui) {
        ui.heading("Recibir un Correo Nuevo");
        ui.add_space(SPACE_MD);

        ui.label("Nombre del remitente");
        ui.text_edit_singleline(&mut self.nuevo_remitente);
        ui.add_space(SPACE_MD);

        ui.label("Asunto del correo");
        ui.text_edit_singleline(&mut self.nuevo_asunto);
        ui.add_space(SPACE_MD);

        ui.label("Cuerpo del mensaje");
        ui.text_edit_singleline(&mut self.nuevo_cuerpo);
        ui.add_space(SPACE_LG);

        if ui
            .add_sized(SUBMIT_BUTTON_SIZE, egui::Button::new("RECIBIR CORREO"))
            .clicked()
        {
            if self.nuevo_remitente.trim().is_empty() {
                self.set_error("El nombre del remitente no puede estar vacío.");
            } else {
                let ahora = Local::now();
                let correo = Correo::new(
                    self.nuevo_remitente.trim(),
                    if self.nuevo_asunto.trim().is_empty() {
                        "Sin Asunto"
                    } else {
                        self.nuevo_asunto.trim()
                    },
                    ahora.date_naive(),
                    ahora.time(),
                    self.nuevo_cuerpo.trim(),
                );
                self.manager.lista.agregar_inicio(correo);
                self.nuevo_remitente.clear();
                self.nuevo_asunto.clear();
                self.nuevo_cuerpo.clear();
                self.seleccion = Some(0);
                self.view = 0;
                self.set_result("Correo recibido e insertado al inicio de la lista.");
            }
        }
        self.show_feedback(ui);
    }

    fn show_feedback(&self, ui: &mut egui::Ui) {
        ui.add_space(SPACE_LG);
        if !self.resultado.is_empty() {
            ui.label(egui::RichText::new(&self.resultado).color(egui::Color32::DARK_GREEN));
        }
        if !self.error.is_empty() {
            ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
        }
    }
}

impl eframe::App for CorreoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gestor de Bandeja de Correos - UABC");
            ui.add_space(8.0);
            self.show_tabs(ui);
            ui.add_space(SPACE_LG);

            match self.view {
                0 => self.show_inbox(ui),
                1 => self.show_new_mail(ui),
                _ => self.show_inbox(ui),
            }

            if self.is_exiting {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
