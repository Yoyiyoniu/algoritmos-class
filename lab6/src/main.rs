mod email;
mod list;

use eframe::egui;
use email::{Correo, CorreoManager};

const WINDOW_SIZE: [f32; 2] = [960.0, 640.0];
const VIEWS: [&str; 2] = ["Bandeja de Entrada", "Recibir Correo"];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Bandeja de Correos - Práctica 6",
        options,
        Box::new(|_cc| Ok(Box::new(CorreoApp::default()))),
    )
}

struct CorreoApp {
    manager: CorreoManager,
    view: usize,

    indice_inicio: usize,
    seleccion_relativa: Option<usize>,

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
            indice_inicio: 0,
            seleccion_relativa: Some(0),
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
                if ui.add_sized([90.0, 30.0], btn).clicked() {
                    self.is_exiting = true;
                }
            });
        });
    }

    fn show_inbox(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bandeja de Entrada (Listas Doblemente Enlazadas)");
        ui.add_space(6.0);

        let total = self.manager.lista.tamano();
        let fin = (self.indice_inicio + 10).min(total);

        // Barra de navegación de la ventana de 10 elementos
        ui.horizontal(|ui| {
            if ui.button("▲ Anterior (Más reciente)").clicked() && self.indice_inicio > 0 {
                self.indice_inicio -= 1;
            }
            ui.label(format!(
                "Mostrando correos {} al {} (Total: {})",
                self.indice_inicio + 1,
                fin,
                total
            ));
            if ui.button("▼ Siguiente (Más antiguo)").clicked() && self.indice_inicio + 10 < total
            {
                self.indice_inicio += 1;
            }
        });

        ui.add_space(8.0);

        // Vista dividida: Lista a la izquierda | Detalle a la derecha
        ui.columns(2, |cols| {
            // Columna Izquierda: Correos visibles
            cols[0].vertical(|ui| {
                ui.label(egui::RichText::new("Seleccione un correo:").strong());
                ui.separator();

                for i in self.indice_inicio..fin {
                    if let Some(nodo) = self.manager.lista.obtener_nodo_por_indice(i) {
                        let correo = nodo.borrow().dato.clone();
                        let rel_idx = i - self.indice_inicio;
                        let label = format!("[{}] {}", i + 1, correo.encabezado_lista());

                        let is_selected = self.seleccion_relativa == Some(rel_idx);
                        if ui.selectable_label(is_selected, label).clicked() {
                            self.seleccion_relativa = Some(rel_idx);
                        }
                    }
                }
            });

            // Columna Derecha: Hilo de conversación y acciones de respuesta
            cols[1].vertical(|ui| {
                ui.label(egui::RichText::new("Hilo de Conversación:").strong());
                ui.separator();

                if let Some(rel_idx) = self.seleccion_relativa {
                    let abs_idx = self.indice_inicio + rel_idx;
                    if let Some(nodo) = self.manager.lista.obtener_nodo_por_indice(abs_idx) {
                        let correo = nodo.borrow().dato.clone();

                        ui.label(format!("Asunto: {}", correo.asunto));
                        ui.label(format!("Remitente: {}", correo.remitente_mostrado));
                        ui.label(format!("Mensajes: {}", correo.contador));
                        ui.add_space(6.0);

                        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                            for cuerpo in &correo.cuerpos {
                                ui.label(cuerpo);
                                ui.separator();
                            }
                        });

                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Contestar como Usuario (Yo):").small());
                        ui.text_edit_singleline(&mut self.texto_respuesta_usuario);
                        if ui.button("Enviar Respuesta (Yo)").clicked() {
                            if !self.texto_respuesta_usuario.trim().is_empty() {
                                nodo.borrow_mut().dato.contestar_por_usuario(self.texto_respuesta_usuario.trim());
                                self.texto_respuesta_usuario.clear();
                                self.set_result("Respuesta enviada. Se agregó ', yo' y se incrementó el contador.");
                            }
                        }

                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Simular respuesta de Remitente Original:").small());
                        ui.text_edit_singleline(&mut self.texto_respuesta_remitente);
                        if ui.button("Recibir Respuesta de Remitente").clicked() {
                            if !self.texto_respuesta_remitente.trim().is_empty() {
                                nodo.borrow_mut().dato.responder_por_remitente(self.texto_respuesta_remitente.trim());
                                self.manager.lista.mover_al_inicio(abs_idx);
                                self.indice_inicio = 0;
                                self.seleccion_relativa = Some(0);
                                self.texto_respuesta_remitente.clear();
                                self.set_result("Respuesta recibida. El correo se movió al INICIO de la lista.");
                            }
                        }
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
        ui.add_space(8.0);

        ui.label("Nombre del remitente");
        ui.text_edit_singleline(&mut self.nuevo_remitente);
        ui.add_space(8.0);

        ui.label("Asunto del correo");
        ui.text_edit_singleline(&mut self.nuevo_asunto);
        ui.add_space(8.0);

        ui.label("Cuerpo del mensaje");
        ui.text_edit_singleline(&mut self.nuevo_cuerpo);
        ui.add_space(12.0);

        if ui
            .add_sized([160.0, 32.0], egui::Button::new("RECIBIR CORREO"))
            .clicked()
        {
            if self.nuevo_remitente.trim().is_empty() {
                self.set_error("El nombre del remitente no puede estar vacío.");
            } else {
                let hoy = chrono::Local::now().date_naive();
                let hora = chrono::Local::now().time();
                let correo = Correo::new(
                    self.nuevo_remitente.trim(),
                    if self.nuevo_asunto.trim().is_empty() {
                        "Sin Asunto"
                    } else {
                        self.nuevo_asunto.trim()
                    },
                    hoy,
                    hora,
                    self.nuevo_cuerpo.trim(),
                );
                self.manager.lista.agregar_inicio(correo);
                self.nuevo_remitente.clear();
                self.nuevo_asunto.clear();
                self.nuevo_cuerpo.clear();
                self.indice_inicio = 0;
                self.seleccion_relativa = Some(0);
                self.view = 0;
                self.set_result("Correo recibido e insertado al inicio de la lista.");
            }
        }
        self.show_feedback(ui);
    }

    fn show_feedback(&self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        if !self.resultado.is_empty() {
            ui.label(egui::RichText::new(&self.resultado).color(egui::Color32::DARK_GREEN));
        }
        if !self.error.is_empty() {
            ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
        }
    }

    fn show_exit_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Confirmación de Salida")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("¿Desea cerrar la aplicación de bandeja de correos?");
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Cerrar").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Cancelar").clicked() {
                        self.is_exiting = false;
                    }
                });
            });
    }
}

impl eframe::App for CorreoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gestor de Bandeja de Correos - UABC");
            ui.add_space(8.0);
            self.show_tabs(ui);
            ui.add_space(12.0);

            match self.view {
                0 => self.show_inbox(ui),
                1 => self.show_new_mail(ui),
                _ => self.show_inbox(ui),
            }

            if self.is_exiting {
                self.show_exit_dialog(ctx);
            }
        });
    }
}
