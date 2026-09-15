mod converter;
mod stack;

use converter::convert_general;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Conversion de numeros fraccionarios entre sistemas numericos",
        options,
        Box::new(|_cc| Ok(Box::new(ConvertApp::default()))),
    )
}

struct ConvertApp {
    numero: String,
    base_orig: u32,
    base_dest: u32,
    precision: u32,
    resultado: String,
    error: String,
}

const BASES: [(u32, &str); 4] = [
    (10, "Decimal (10)"),
    (2, "Binario (2)"),
    (8, "Octal (8)"),
    (16, "Hexadecimal (16)"),
];

const WINDOW_SIZE: [f32; 2] = [900.0, 420.0];
const PRECISION_MIN: u32 = 1;
const PRECISION_MAX: u32 = 16;

fn base_name(base: u32) -> &'static str {
    match base {
        2 => "Binario (2)",
        8 => "Octal (8)",
        16 => "Hexadecimal (16)",
        _ => "Decimal (10)",
    }
}

impl Default for ConvertApp {
    fn default() -> Self {
        Self {
            numero: "6.1".to_owned(),
            base_orig: 10,
            base_dest: 2,
            precision: 8,
            resultado: String::new(),
            error: String::new(),
        }
    }
}

impl ConvertApp {
    fn set_error(&mut self, message: &str) {
        self.resultado.clear();
        self.error = message.to_owned();
    }

    fn calculate(&mut self) {
        self.error.clear();
        match convert_general(
            &self.numero,
            self.base_orig,
            self.base_dest,
            self.precision as usize,
        ) {
            Ok(out) => self.resultado = out,
            Err(e) => self.set_error(&e),
        }
    }

    fn show_number_inputs(&mut self, ui: &mut egui::Ui) {
        ui.columns(3, |cols| {
            cols[0].vertical(|ui| {
                ui.label("Numero original");
                ui.text_edit_singleline(&mut self.numero);
            });
            cols[1].vertical(|ui| {
                show_base_selector(
                    ui,
                    "Base original",
                    "Base del sistema numerico original",
                    "base_orig",
                    &mut self.base_orig,
                );
            });
            cols[2].vertical(|ui| {
                show_base_selector(
                    ui,
                    "Base de resultados",
                    "Base del sistema numerico resultante",
                    "base_dest",
                    &mut self.base_dest,
                );
            });
        });
    }

    fn show_precision_and_action(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Digitos despues del punto decimal:");
            ui.add(egui::Slider::new(&mut self.precision, PRECISION_MIN..=PRECISION_MAX).text(""));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    egui::RichText::new("CALCULAR")
                        .color(egui::Color32::WHITE)
                        .strong(),
                )
                .fill(egui::Color32::from_rgb(255, 109, 0));
                if ui.add_sized([110.0, 32.0], btn).clicked() {
                    self.calculate();
                }
            });
        });
    }

    fn show_output(&self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Numero resultante").small().weak());
        if !self.resultado.is_empty() {
            ui.label(egui::RichText::new(&self.resultado).monospace().size(20.0));
        }
        if !self.error.is_empty() {
            ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
        }
    }
}

impl eframe::App for ConvertApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Conversion de numeros fraccionarios entre sistemas numericos");
            ui.add_space(12.0);

            self.show_number_inputs(ui);
            ui.add_space(12.0);
            self.show_precision_and_action(ui);
            ui.add_space(24.0);
            self.show_output(ui);
        });
    }
}

/// ComboBox de base con las opciones de [`BASES`].
/// El `id_salt` debe ser distinto por selector para no colisionar
/// el estado interno de egui.
fn show_base_selector(ui: &mut egui::Ui, label: &str, hint: &str, id_salt: &str, base: &mut u32) {
    ui.label(label);
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(base_name(*base))
        .show_ui(ui, |ui| {
            for (value, option_label) in BASES {
                ui.selectable_value(base, value, option_label);
            }
        });
    ui.label(egui::RichText::new(hint).small().weak());
}
