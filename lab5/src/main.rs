mod list;
mod shopping;

use eframe::egui;
use shopping::{CATEGORIES, PLACES, ShoppingManager};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Gestor de compras de supermercado",
        options,
        Box::new(|_cc| Ok(Box::new(ShoppingApp::default()))),
    )
}

struct ShoppingApp {
    manager: ShoppingManager,
    view: usize,
    reg_name: String,
    reg_quantity: String,
    reg_place: usize,
    reg_category: usize,
    buy_selected: usize,
    buy_quantity: String,
    buy_other_place: bool,
    buy_place: usize,
    filter_category: usize,
    filter_place: usize,
    resultado: String,
    error: String,
    is_exiting: bool,
}

const WINDOW_SIZE: [f32; 2] = [960.0, 640.0];
const VIEWS: [&str; 4] = ["Registrar", "Comprar", "Clasificar", "Comprados"];

impl Default for ShoppingApp {
    fn default() -> Self {
        Self {
            manager: ShoppingManager::new(),
            view: 0,
            reg_name: String::new(),
            reg_quantity: String::new(),
            reg_place: 0,
            reg_category: 0,
            buy_selected: 0,
            buy_quantity: String::new(),
            buy_other_place: false,
            buy_place: 0,
            filter_category: 0,
            filter_place: 0,
            resultado: String::new(),
            error: String::new(),
            is_exiting: false,
        }
    }
}

impl ShoppingApp {
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

    fn show_register(&mut self, ui: &mut egui::Ui) {
        ui.heading("Registrar artículo pendiente");
        ui.add_space(8.0);
        ui.columns(2, |cols| {
            cols[0].vertical(|ui| {
                ui.label("Nombre del producto");
                ui.text_edit_singleline(&mut self.reg_name);
                ui.add_space(8.0);
                ui.label("Cantidad planeada");
                ui.text_edit_singleline(&mut self.reg_quantity);
            });
            cols[1].vertical(|ui| {
                show_option_selector(
                    ui,
                    "Lugar de compra planeado",
                    "Seleccione el establecimiento",
                    "reg_place",
                    &PLACES,
                    &mut self.reg_place,
                );
                show_option_selector(
                    ui,
                    "Categoría",
                    "Seleccione la categoría",
                    "reg_category",
                    &CATEGORIES,
                    &mut self.reg_category,
                );
            });
        });
        ui.add_space(12.0);
        if ui.add_sized([140.0, 32.0], egui::Button::new("REGISTRAR")).clicked() {
            match self.manager.register(
                &self.reg_name,
                &self.reg_quantity,
                self.reg_place,
                self.reg_category,
            ) {
                Ok(()) => {
                    self.reg_name.clear();
                    self.reg_quantity.clear();
                    self.set_result("Artículo registrado en pendientes.");
                }
                Err(message) => self.set_error(&message),
            }
        }
        self.show_feedback(ui);
    }

    fn show_buy(&mut self, ui: &mut egui::Ui) {
        ui.heading("Comprar artículo pendiente");
        ui.add_space(8.0);
        if self.manager.pending_len() == 0 {
            ui.label(egui::RichText::new("No hay artículos pendientes.").weak());
            return;
        }
        self.buy_selected = self.buy_selected.min(self.manager.pending_len() - 1);
        let labels: Vec<String> = (0..self.manager.pending_len())
            .filter_map(|index| self.manager.pending_label(index))
            .collect();
        let mut selected = self.buy_selected;
        egui::ComboBox::from_id_salt("buy_item")
            .selected_text(labels.get(selected).cloned().unwrap_or_default())
            .show_ui(ui, |ui| {
                for (index, label) in labels.iter().enumerate() {
                    ui.selectable_value(&mut selected, index, label);
                }
            });
        self.buy_selected = selected;
        ui.add_space(8.0);
        ui.label("Cantidad comprada");
        ui.text_edit_singleline(&mut self.buy_quantity);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.buy_other_place, false, "Lugar planeado");
            ui.radio_value(&mut self.buy_other_place, true, "Otro lugar");
        });
        if self.buy_other_place {
            show_option_selector(
                ui,
                "Lugar real de compra",
                "Seleccione el establecimiento real",
                "buy_place",
                &PLACES,
                &mut self.buy_place,
            );
        }
        ui.add_space(8.0);
        if ui.add_sized([140.0, 32.0], egui::Button::new("COMPRAR")).clicked() {
            let real_place = self.buy_other_place.then_some(self.buy_place);
            match self.manager.buy(self.buy_selected, &self.buy_quantity, real_place) {
                Ok(()) => {
                    self.buy_quantity.clear();
                    self.buy_other_place = false;
                    self.buy_place = 0;
                    self.buy_selected = 0;
                    self.set_result("Compra registrada.");
                }
                Err(message) => self.set_error(&message),
            }
        }
        self.show_feedback(ui);
    }

    fn show_classify(&mut self, ui: &mut egui::Ui) {
        ui.heading("Clasificar pendientes");
        ui.add_space(8.0);
        ui.columns(2, |cols| {
            cols[0].vertical(|ui| {
                show_option_selector(
                    ui,
                    "Categoría",
                    "Filtre por categoría",
                    "filter_category",
                    &CATEGORIES,
                    &mut self.filter_category,
                );
            });
            cols[1].vertical(|ui| {
                show_option_selector(
                    ui,
                    "Lugar planeado",
                    "Filtre por establecimiento",
                    "filter_place",
                    &PLACES,
                    &mut self.filter_place,
                );
            });
        });
        ui.add_space(12.0);
        match self.manager.classified(self.filter_category, self.filter_place) {
            Ok(items) => {
                if items.is_empty() {
                    ui.label(egui::RichText::new("Sin pendientes con ese filtro.").weak());
                } else {
                    for item in items {
                        ui.label(format!(
                            "{} | quedan {} | {} ({})",
                            item.name,
                            item.pending_quantity,
                            item.place,
                            item.category
                        ));
                    }
                }
            }
            Err(message) => self.set_error(&message),
        }
        self.show_feedback(ui);
    }

    fn show_bought(&mut self, ui: &mut egui::Ui) {
        ui.heading("Artículos comprados");
        ui.add_space(8.0);
        if self.manager.bought_len() == 0 {
            ui.label(egui::RichText::new("Aún no hay compras registradas.").weak());
            return;
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for item in self.manager.bought() {
                ui.label(egui::RichText::new(&item.name).strong());
                ui.label(item.place_detail());
                ui.label(item.quantity_detail());
                ui.label(egui::RichText::new(item.category).small().weak());
                ui.separator();
            }
        });
    }

    fn show_feedback(&self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        if !self.resultado.is_empty() {
            ui.label(
                egui::RichText::new(&self.resultado).color(egui::Color32::DARK_GREEN),
            );
        }
        if !self.error.is_empty() {
            ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
        }
    }

    fn show_exit_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pendientes por comprar")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if self.manager.pending_len() == 0 {
                    ui.label("No quedaron pendientes. Todo comprado.");
                } else {
                    for item in self.manager.pending() {
                        ui.label(format!(
                            "{} | quedan {} | {} ({})",
                            item.name,
                            item.pending_quantity,
                            item.place,
                            item.category
                        ));
                    }
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Salir").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Seguir comprando").clicked() {
                        self.is_exiting = false;
                    }
                });
            });
    }
}

impl eframe::App for ShoppingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gestor de compras de supermercado");
            ui.add_space(8.0);
            self.show_tabs(ui);
            ui.add_space(12.0);
            match self.view {
                0 => self.show_register(ui),
                1 => self.show_buy(ui),
                2 => self.show_classify(ui),
                _ => self.show_bought(ui),
            }
            if self.is_exiting {
                self.show_exit_dialog(ctx);
            }
        });
    }
}

fn show_option_selector(
    ui: &mut egui::Ui,
    label: &str,
    hint: &str,
    id_salt: &str,
    options: &[&str],
    selected: &mut usize,
) {
    let current = options.get(*selected).copied().unwrap_or_default();
    ui.label(label);
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(current)
        .show_ui(ui, |ui| {
            for (index, option) in options.iter().enumerate() {
                ui.selectable_value(selected, index, *option);
            }
        });
    ui.label(egui::RichText::new(hint).small().weak());
}
