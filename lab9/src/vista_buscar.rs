use crate::app::{CriterioBusqueda, EstudiantesApp, SPACE_MD, SPACE_SM, ficha};
use crate::estudiante::Algoritmo;
use crate::vista_alta::parsear_matricula;
use eframe::egui;

pub(crate) fn show(app: &mut EstudiantesApp, ui: &mut egui::Ui) {
    ui.heading("Buscar estudiante");
    ui.label(
        egui::RichText::new(
            "Por nombre se usa búsqueda binaria; por matrícula, interpolada. También puedes comparar con la lineal.",
        )
        .small()
        .weak(),
    );
    ui.add_space(SPACE_MD);

    ui.horizontal(|ui| {
        ui.label("Buscar por:");
        let mut cambio = ui
            .selectable_value(&mut app.criterio, CriterioBusqueda::Nombre, "Nombre")
            .clicked();
        cambio |= ui
            .selectable_value(&mut app.criterio, CriterioBusqueda::Matricula, "Matrícula")
            .clicked();
        if cambio {
            app.algoritmo = match app.criterio {
                CriterioBusqueda::Nombre => Algoritmo::Binaria,
                CriterioBusqueda::Matricula => Algoritmo::Interpolada,
            };
            app.consulta.clear();
            app.ficha.clear();
            app.error.clear();
            app.resultado.clear();
        }

        ui.add_space(SPACE_MD);
        ui.label("Algoritmo:");
        egui::ComboBox::from_id_salt("combo_algoritmo")
            .selected_text(app.algoritmo.nombre())
            .show_ui(ui, |ui| {
                let opciones: &[Algoritmo] = match app.criterio {
                    CriterioBusqueda::Nombre => &[Algoritmo::Binaria, Algoritmo::Lineal],
                    CriterioBusqueda::Matricula => &[
                        Algoritmo::Interpolada,
                        Algoritmo::Binaria,
                        Algoritmo::Lineal,
                    ],
                };
                for alg in opciones {
                    ui.selectable_value(&mut app.algoritmo, *alg, alg.nombre());
                }
            });
    });
    ui.add_space(SPACE_SM);

    ui.horizontal(|ui| {
        ui.label(match app.criterio {
            CriterioBusqueda::Nombre => "Nombre completo:",
            CriterioBusqueda::Matricula => "Matrícula:",
        });
        ui.add(
            egui::TextEdit::singleline(&mut app.consulta)
                .desired_width(280.0)
                .hint_text(match app.criterio {
                    CriterioBusqueda::Nombre => "Nombre Apellido Apellido",
                    CriterioBusqueda::Matricula => "Ej: 15639",
                }),
        );
        if ui.button("🔍 BUSCAR").clicked() {
            ejecutar(app);
        }
    });

    ui.add_space(SPACE_MD);
    if !app.ficha.is_empty() {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .show(ui, |ui| {
                ui.label(egui::RichText::new(&app.ficha).monospace());
            });
    }

    app.mostrar_feedback(ui);
}

fn ejecutar(app: &mut EstudiantesApp) {
    app.ficha.clear();
    if app.consulta.trim().is_empty() {
        app.set_error("Escribe un valor para buscar.");
        return;
    }

    let busqueda = match app.criterio {
        CriterioBusqueda::Nombre => app.registro.buscar_nombre(&app.consulta, app.algoritmo),
        CriterioBusqueda::Matricula => match parsear_matricula(&app.consulta) {
            Ok(m) => app.registro.buscar_matricula(m, app.algoritmo),
            Err(mensaje) => {
                app.set_error(&mensaje);
                return;
            }
        },
    };

    // La interpolada no aplica a nombres: se informa el algoritmo realmente usado.
    let usado = match (app.criterio, app.algoritmo) {
        (CriterioBusqueda::Nombre, Algoritmo::Interpolada) => Algoritmo::Binaria,
        (_, alg) => alg,
    };

    match busqueda.indice {
        Some(indice) => {
            app.ficha = ficha(app.registro.obtener(indice));
            app.set_result(&format!(
                "Registro encontrado (búsqueda {}, {} pasos).",
                usado.nombre().to_lowercase(),
                busqueda.pasos
            ));
        }
        None => app.set_error(&format!(
            "No existe el registro «{}» (búsqueda {}, {} pasos).",
            app.consulta.trim(),
            usado.nombre().to_lowercase(),
            busqueda.pasos
        )),
    }
}
