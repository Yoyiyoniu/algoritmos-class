use crate::app::{EstudiantesApp, SPACE_LG, SPACE_MD, SPACE_SM, SUBMIT_BUTTON_SIZE, ficha};
use crate::vista_alta::{parsear_grado, parsear_matricula, parsear_promedio};
use eframe::egui;

pub(crate) fn show(app: &mut EstudiantesApp, ui: &mut egui::Ui) {
    ui.heading("Modificar estudiante");
    ui.label(
        egui::RichText::new(
            "Se localiza por matrícula (búsqueda interpolada). Deja vacío lo que no quieras cambiar.",
        )
        .small()
        .weak(),
    );
    ui.add_space(SPACE_MD);

    egui::Grid::new("grid_modificar")
        .num_columns(2)
        .spacing([SPACE_LG, SPACE_SM])
        .show(ui, |ui| {
            ui.label("Matrícula (no modificable)");
            ui.text_edit_singleline(&mut app.matricula);
            ui.end_row();

            ui.label("Nuevo nombre");
            ui.text_edit_singleline(&mut app.nombre);
            ui.end_row();

            ui.label("Nuevo grado escolar");
            ui.text_edit_singleline(&mut app.grado);
            ui.end_row();

            ui.label("Nuevo promedio");
            ui.text_edit_singleline(&mut app.promedio);
            ui.end_row();
        });

    ui.add_space(SPACE_LG);
    if ui
        .add_sized(SUBMIT_BUTTON_SIZE, egui::Button::new("✏ MODIFICAR"))
        .clicked()
    {
        match aplicar(app) {
            Ok(mensaje) => {
                app.set_result(&mensaje);
                app.limpiar_campos();
            }
            Err(mensaje) => {
                app.ficha.clear();
                app.set_error(&mensaje);
            }
        }
    }

    if !app.ficha.is_empty() {
        ui.add_space(SPACE_MD);
        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .show(ui, |ui| {
                ui.label(egui::RichText::new(&app.ficha).monospace());
            });
    }

    app.mostrar_feedback(ui);
}

fn aplicar(app: &mut EstudiantesApp) -> Result<String, String> {
    if app.matricula.trim().is_empty() {
        return Err("Escribe la matrícula del estudiante a modificar.".to_owned());
    }
    let matricula = parsear_matricula(&app.matricula)?;

    let nombre = app.nombre.trim();
    let nuevo_nombre = (!nombre.is_empty()).then(|| nombre.to_owned());
    let nuevo_grado = (!app.grado.trim().is_empty())
        .then(|| parsear_grado(&app.grado))
        .transpose()?;
    let nuevo_promedio = (!app.promedio.trim().is_empty())
        .then(|| parsear_promedio(&app.promedio))
        .transpose()?;

    if nuevo_nombre.is_none() && nuevo_grado.is_none() && nuevo_promedio.is_none() {
        return Err("Debes llenar al menos uno de los campos a modificar.".to_owned());
    }

    let busqueda = app.registro.buscar_interpolada(matricula);
    let Some(indice) = busqueda.indice else {
        return Err(format!(
            "No existe ningún estudiante con la matrícula {matricula}."
        ));
    };

    app.registro
        .modificar(indice, nuevo_nombre, nuevo_grado, nuevo_promedio);
    let indice = app
        .registro
        .buscar_interpolada(matricula)
        .indice
        .unwrap_or(indice);
    app.ficha = ficha(app.registro.obtener(indice));
    Ok(format!(
        "Registro {matricula} actualizado (interpolada: {} pasos).",
        busqueda.pasos
    ))
}
