mod app;
mod arbol;

use app::CalculadoraApp;
use eframe::egui;

const WINDOW_SIZE: [f32; 2] = [420.0, 600.0];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Calculadora - Árbol de Expresión",
        options,
        Box::new(|_cc| Ok(Box::new(CalculadoraApp::default()))),
    )
}
