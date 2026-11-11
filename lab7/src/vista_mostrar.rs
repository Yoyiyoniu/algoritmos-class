use crate::app::{FILTROS, GimnasioApp, SPACE_LG, SPACE_MD, SPACE_SM, ficha};
use crate::avl::Membresia;
use eframe::egui;

const REGRESAR_INDICE: usize = 3;

pub(crate) fn show(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.heading("Mostrar");
    ui.add_space(SPACE_SM);

    ui.horizontal_wrapped(|ui| {
        for (index, nombre) in FILTROS.iter().enumerate() {
            if ui
                .selectable_value(&mut app.mostrar_indice, index, *nombre)
                .clicked()
            {
                if index == REGRESAR_INDICE {
                    app.view = 0;
                }
                app.ficha_cliente.clear();
                app.error.clear();
                app.resultado.clear();
            }
        }
    });

    ui.add_space(SPACE_MD);
    match app.mostrar_indice {
        0 => listado_general(app, ui),
        1 => filtrado_por_membresia(app, ui),
        2 => individual(app, ui),
        _ => {}
    }

    app.mostrar_feedback(ui);
}

fn listado_general(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    let clientes = app.arbol.listar();
    ui.label(egui::RichText::new("Listado general (nombre, teléfono, membresía)").strong());
    ui.add_space(SPACE_SM);

    egui::Grid::new("grid_listado")
        .num_columns(3)
        .striped(true)
        .spacing([SPACE_LG, SPACE_SM])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Nombre").strong());
            ui.label(egui::RichText::new("Teléfono").strong());
            ui.label(egui::RichText::new("Membresía").strong());
            ui.end_row();

            for cliente in &clientes {
                ui.label(&cliente.nombre);
                ui.label(&cliente.telefono);
                ui.label(cliente.membresia.nombre());
                ui.end_row();
            }
        });

    ui.add_space(SPACE_SM);
    ui.label(format!("Total de clientes en el árbol: {}", clientes.len()));
}

fn filtrado_por_membresia(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Tipo de membresía:");
        egui::ComboBox::from_id_salt("combo_filtro")
            .selected_text(Membresia::TODAS[app.filtro_membresia].nombre())
            .show_ui(ui, |ui| {
                for (index, membresia) in Membresia::TODAS.iter().enumerate() {
                    ui.selectable_value(&mut app.filtro_membresia, index, membresia.nombre());
                }
            });
    });
    ui.add_space(SPACE_MD);

    let clientes = app
        .arbol
        .filtrar_por_membresia(Membresia::TODAS[app.filtro_membresia]);
    ui.label(egui::RichText::new("Clientes con esa membresía (nombre, teléfono)").strong());
    ui.add_space(SPACE_SM);

    if clientes.is_empty() {
        ui.label("No hay clientes con esa membresía.");
        return;
    }

    egui::Grid::new("grid_filtrado")
        .num_columns(2)
        .striped(true)
        .spacing([SPACE_LG, SPACE_SM])
        .show(ui, |ui| {
            for cliente in &clientes {
                ui.label(&cliente.nombre);
                ui.label(&cliente.telefono);
                ui.end_row();
            }
        });

    ui.add_space(SPACE_SM);
    ui.label(format!("Clientes encontrados: {}", clientes.len()));
}

fn individual(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Nombre completo:");
        ui.add(
            egui::TextEdit::singleline(&mut app.nombre)
                .desired_width(280.0)
                .hint_text("Apellido Nombre"),
        );
        if ui.button("BUSCAR").clicked() {
            match app.arbol.buscar(&app.nombre).cloned() {
                Some(cliente) => {
                    app.set_result(&format!("Cliente encontrado: {}.", cliente.nombre));
                    app.ficha_cliente = ficha(&cliente);
                }
                None => {
                    app.ficha_cliente.clear();
                    app.set_error(&format!(
                        "No se encontró ningún cliente con el nombre {}.",
                        app.nombre.trim()
                    ));
                }
            }
        }
    });

    ui.add_space(SPACE_MD);
    if !app.ficha_cliente.is_empty() {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .show(ui, |ui| {
                ui.label(egui::RichText::new(&app.ficha_cliente).monospace());
            });
    }
}
