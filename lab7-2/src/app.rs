use crate::arbol::calcular;
use eframe::egui;

const SPACE_SM: f32 = 6.0;
const SPACE_MD: f32 = 8.0;
const TECLA_SIZE: [f32; 2] = [70.0, 52.0];
const REGLA_ALTO: f32 = 36.0;
const ROJO: egui::Color32 = egui::Color32::from_rgb(200, 40, 40);
const VERDE: egui::Color32 = egui::Color32::from_rgb(40, 120, 40);

const TECLADO: [[char; 4]; 5] = [
    ['C', '(', ')', 'X'],
    ['7', '8', '9', '/'],
    ['4', '5', '6', '*'],
    ['1', '2', '3', '-'],
    ['^', '!', '0', '+'],
];

#[derive(Default)]
pub(crate) struct CalculadoraApp {
    pub(crate) expresion: String,

    resultado: String,

    formateada: String,

    es_error: bool,
}

impl CalculadoraApp {
    fn resolver(&mut self) {
        self.resultado.clear();
        self.formateada.clear();
        match calcular(&self.expresion) {
            Ok((valor, formateada)) => {
                self.es_error = false;
                self.resultado = format!("{valor:?}");
                self.formateada = formateada;
            }
            Err(()) => {
                self.es_error = true;
                self.resultado = "syntax error".to_owned();
            }
        }
    }

    fn teclear(&mut self, tecla: char) {
        match tecla {
            'C' => {
                self.expresion.clear();
                self.limpiar_salida();
            }
            '⌫' => {
                self.expresion.pop();
                self.limpiar_salida();
            }
            '=' => self.resolver(),
            t => {
                self.expresion.push(t);
                self.limpiar_salida();
            }
        }
    }

    fn limpiar_salida(&mut self) {
        self.resultado.clear();
        self.formateada.clear();
        self.es_error = false;
    }
}

impl eframe::App for CalculadoraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calculadora — Árbol binario de expresión (UABC)");
            ui.add_space(SPACE_MD);

            ui.add_sized(
                [ui.available_width(), REGLA_ALTO],
                egui::TextEdit::singleline(&mut self.expresion)
                    .font(egui::TextStyle::Heading)
                    .hint_text("escribe o usa las teclas…"),
            );

            ui.add_space(SPACE_SM);
            let color = if self.es_error { ROJO } else { VERDE };
            ui.add_sized(
                [ui.available_width(), REGLA_ALTO],
                egui::Label::new(egui::RichText::new(&self.resultado).color(color).strong()),
            );

            if self.formateada.is_empty() {
                ui.add_space(SPACE_SM);
            } else {
                ui.label(
                    egui::RichText::new(format!("Formateada: {}", self.formateada))
                        .small()
                        .color(egui::Color32::GRAY),
                );
            }

            ui.separator();
            ui.add_space(SPACE_SM);

            egui::Grid::new("teclado")
                .spacing([SPACE_MD, SPACE_SM])
                .show(ui, |ui| {
                    for fila in TECLADO {
                        for tecla in fila {
                            if ui
                                .add_sized(TECLA_SIZE, egui::Button::new(tecla.to_string()))
                                .clicked()
                            {
                                self.teclear(tecla);
                            }
                        }
                        ui.end_row();
                    }
                    if ui
                        .add_sized(
                            [ui.available_width(), 44.0],
                            egui::Button::new(egui::RichText::new("=").size(20.0).strong()),
                        )
                        .clicked()
                    {
                        self.teclear('=');
                    }
                    ui.end_row();
                });
        });
    }
}
