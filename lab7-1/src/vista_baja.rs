use crate::app::{GimnasioApp, ROJO, SPACE_MD, SUBMIT_BUTTON_SIZE};
use eframe::egui;

pub(crate) fn show(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.heading("Baja del contacto");
    ui.add_space(SPACE_MD);

    ui.horizontal(|ui| {
        ui.label("Nombre completo:");
        ui.add(
            egui::TextEdit::singleline(&mut app.nombre)
                .desired_width(280.0)
                .hint_text("Apellido Nombre"),
        );
        if ui
            .add_sized(
                SUBMIT_BUTTON_SIZE,
                egui::Button::new(egui::RichText::new("DAR DE BAJA").color(egui::Color32::WHITE))
                    .fill(ROJO),
            )
            .clicked()
        {
            let nombre = app.nombre.trim().to_owned();
            if app.arbol.eliminar(&nombre) {
                app.nombre.clear();
                app.set_result(&format!(
                    "Cliente dado de baja: {}. Total de clientes: {}.",
                    nombre,
                    app.arbol.total()
                ));
            } else {
                app.set_error(&format!(
                    "No se encontró ningún cliente con el nombre {nombre}."
                ));
            }
        }
    });

    app.mostrar_feedback(ui);
}
