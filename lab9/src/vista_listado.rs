use crate::app::{EstudiantesApp, SPACE_LG, SPACE_MD, SPACE_SM};
use crate::estudiante::Filtros;
use crate::vista_alta::{parsear_grado, parsear_matricula};
use eframe::egui;

pub(crate) fn show(app: &mut EstudiantesApp, ui: &mut egui::Ui) {
    ui.heading("Listado de registros");
    ui.add_space(SPACE_SM);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(egui::RichText::new("Filtros").strong());
        ui.horizontal(|ui| {
            ui.checkbox(&mut app.filtro_grado_activo, "Grado escolar");
            ui.add_enabled(
                app.filtro_grado_activo,
                egui::TextEdit::singleline(&mut app.filtro_grado)
                    .desired_width(50.0)
                    .hint_text("Ej: 4"),
            );

            ui.add_space(SPACE_LG);
            ui.checkbox(&mut app.filtro_rango_activo, "Rango de matrícula");
            ui.add_enabled_ui(app.filtro_rango_activo, |ui| {
                ui.label("de");
                ui.add(egui::TextEdit::singleline(&mut app.rango_min).desired_width(70.0));
                ui.label("a");
                ui.add(egui::TextEdit::singleline(&mut app.rango_max).desired_width(70.0));
            });
        });
        ui.add_space(SPACE_SM);
        if ui.button("🔍 APLICAR FILTROS").clicked() {
            match leer_filtros(app) {
                Ok(filtros) => {
                    app.filtros_aplicados = Some(filtros);
                    app.error.clear();
                    app.resultado.clear();
                }
                Err(mensaje) => app.set_error(&mensaje),
            }
        }
    });
    ui.add_space(SPACE_MD);

    let Some(filtros) = app.filtros_aplicados else {
        return;
    };
    let filas = app.registro.filtrar(filtros);

    egui::ScrollArea::vertical()
        .max_height(320.0)
        .show(ui, |ui| {
            egui::Grid::new("grid_listado")
                .num_columns(4)
                .striped(true)
                .spacing([SPACE_LG * 2.0, SPACE_SM])
                .show(ui, |ui| {
                    for titulo in ["Matrícula", "Nombre completo", "Grado", "Promedio"] {
                        ui.label(egui::RichText::new(titulo).strong());
                    }
                    ui.end_row();

                    for e in &filas {
                        ui.label(e.matricula.to_string());
                        ui.label(&e.nombre);
                        ui.label(e.grado.to_string());
                        ui.label(format!("{:.1}", e.promedio));
                        ui.end_row();
                    }
                });
        });

    ui.add_space(SPACE_SM);
    if filas.is_empty() {
        ui.label("Ningún registro cumple con los filtros.");
    }
    ui.label(format!(
        "Registros mostrados: {} de {}",
        filas.len(),
        app.registro.total()
    ));

    app.mostrar_feedback(ui);
}

fn leer_filtros(app: &EstudiantesApp) -> Result<Filtros, String> {
    let grado = if app.filtro_grado_activo {
        Some(parsear_grado(&app.filtro_grado)?)
    } else {
        None
    };
    let rango = if app.filtro_rango_activo {
        let min = parsear_matricula(&app.rango_min)
            .map_err(|_| "El inicio del rango debe ser una matrícula de 5 dígitos.".to_owned())?;
        let max = parsear_matricula(&app.rango_max)
            .map_err(|_| "El fin del rango debe ser una matrícula de 5 dígitos.".to_owned())?;
        if min > max {
            return Err("El inicio del rango no puede ser mayor que el fin.".to_owned());
        }
        Some((min, max))
    } else {
        None
    };
    Ok((grado, rango))
}
