use crate::app::{GimnasioApp, SPACE_LG, SPACE_MD, SPACE_SM, SUBMIT_BUTTON_SIZE};
use crate::avl::{Cliente, Membresia};
use eframe::egui;

pub(crate) fn show(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.heading("Alta del cliente");
    ui.label(
        egui::RichText::new(
            "Los datos se validan antes de insertarse en el árbol AVL (clave: nombre).",
        )
        .small()
        .weak(),
    );
    ui.add_space(SPACE_MD);

    egui::Grid::new("grid_alta")
        .num_columns(2)
        .spacing([SPACE_LG, SPACE_SM])
        .show(ui, |ui| {
            ui.label("Nombre completo (apellido primero)");
            ui.text_edit_singleline(&mut app.nombre);
            ui.end_row();

            ui.label("Edad (mayor de 15 años)");
            ui.text_edit_singleline(&mut app.edad);
            ui.end_row();

            ui.label("Teléfono de contacto (celular, 10 dígitos)");
            ui.text_edit_singleline(&mut app.telefono);
            ui.end_row();

            ui.label("Correo electrónico (opcional)");
            ui.text_edit_singleline(&mut app.correo);
            ui.end_row();

            ui.label("Mensualidades atrasadas");
            ui.text_edit_singleline(&mut app.atrasadas);
            ui.end_row();

            ui.label("Nivel de membresía");
            egui::ComboBox::from_id_salt("combo_membresia")
                .selected_text(&app.membresia_texto)
                .show_ui(ui, |ui| {
                    for membresia in Membresia::TODAS {
                        ui.selectable_value(
                            &mut app.membresia_texto,
                            membresia.nombre().to_owned(),
                            membresia.nombre(),
                        );
                    }
                });
            ui.end_row();
        });

    ui.add_space(SPACE_LG);
    if ui
        .add_sized(SUBMIT_BUTTON_SIZE, egui::Button::new("DAR DE ALTA"))
        .clicked()
    {
        match validar(app) {
            Ok(cliente) => {
                let nombre = cliente.nombre.clone();
                app.arbol.insertar(cliente);
                app.set_result(&format!(
                    "Cliente registrado: {}. Total de clientes: {}.",
                    nombre,
                    app.arbol.total()
                ));
                app.limpiar_campos();
            }
            Err(mensaje) => app.set_error(&mensaje),
        }
    }

    app.mostrar_feedback(ui);
}

fn validar(app: &GimnasioApp) -> Result<Cliente, String> {
    let nombre = app.nombre.trim();
    if nombre.is_empty() || !nombre.contains(' ') {
        return Err("El nombre completo debe incluir apellido y nombre.".to_owned());
    }
    if app.arbol.buscar(nombre).is_some() {
        return Err(format!("El cliente {nombre} ya está registrado."));
    }

    let edad: u32 = app
        .edad
        .trim()
        .parse()
        .map_err(|_| "La edad debe ser un número entero.".to_owned())?;
    if edad <= 15 {
        return Err("La edad debe ser mayor de 15 años.".to_owned());
    }

    let telefono = app.telefono.trim();
    if telefono.len() != 10 || !telefono.chars().all(|c| c.is_ascii_digit()) {
        return Err("El teléfono debe tener 10 dígitos.".to_owned());
    }

    let correo = app.correo.trim();
    if !correo.is_empty() && !correo.contains('@') && !correo.contains('.') {
        return Err("El correo electrónico no tiene un formato válido.".to_owned());
    }

    let atrasadas: u32 = app
        .atrasadas
        .trim()
        .parse()
        .map_err(|_| "Las mensualidades atrasadas deben ser un número entero.".to_owned())?;

    let membresia = Membresia::desde_texto(&app.membresia_texto)
        .ok_or_else(|| "El nivel de membresía no es válido.".to_owned())?;

    Ok(Cliente::nuevo(
        nombre,
        edad,
        telefono,
        Some(correo),
        atrasadas,
        membresia,
    ))
}
