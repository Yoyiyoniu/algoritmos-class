mod app;
mod avl;
mod vista_alta;
mod vista_baja;
mod vista_correo;
mod vista_mostrar;

use app::GimnasioApp;
use eframe::egui;

const WINDOW_SIZE: [f32; 2] = [960.0, 640.0];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Gimnasio - Clientes",
        options,
        Box::new(|_cc| Ok(Box::new(GimnasioApp::default()))),
    )
}
