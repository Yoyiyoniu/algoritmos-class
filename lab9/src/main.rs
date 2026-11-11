mod app;
mod estudiante;
mod vista_alta;
mod vista_buscar;
mod vista_listado;
mod vista_modificar;

use app::EstudiantesApp;
use eframe::egui;

const WINDOW_SIZE: [f32; 2] = [960.0, 640.0];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Gestión de Estudiantes",
        options,
        Box::new(|_cc| Ok(Box::new(EstudiantesApp::default()))),
    )
}
