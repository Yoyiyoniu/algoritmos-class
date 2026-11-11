use crate::app::{EstudiantesApp, SPACE_LG, SPACE_MD, SPACE_SM, SUBMIT_BUTTON_SIZE};
use crate::estudiante::{
    Estudiante, GRADO_MAX, GRADO_MIN, MATRICULA_MAX, MATRICULA_MIN, MAX_ESTUDIANTES,
};
use eframe::egui;

pub(crate) fn show(app: &mut EstudiantesApp, ui: &mut egui::Ui) {
    ui.heading("Alta de estudiante");
    ui.label(
        egui::RichText::new(format!(
            "Todos los campos son obligatorios. Máximo {MAX_ESTUDIANTES} registros; la matrícula no puede repetirse."
        ))
        .small()
        .weak(),
    );
    ui.add_space(SPACE_MD);

    egui::Grid::new("grid_alta")
        .num_columns(2)
        .spacing([SPACE_LG, SPACE_SM])
        .show(ui, |ui| {
            ui.label("Nombre completo");
            ui.text_edit_singleline(&mut app.nombre);
            ui.end_row();

            ui.label("Matrícula (5 dígitos)");
            ui.text_edit_singleline(&mut app.matricula);
            ui.end_row();

            ui.label(format!("Grado escolar ({GRADO_MIN}-{GRADO_MAX})"));
            ui.text_edit_singleline(&mut app.grado);
            ui.end_row();

            ui.label("Promedio académico (0-100)");
            ui.text_edit_singleline(&mut app.promedio);
            ui.end_row();
        });

    ui.add_space(SPACE_LG);
    if ui
        .add_sized(SUBMIT_BUTTON_SIZE, egui::Button::new("✔ GUARDAR"))
        .clicked()
    {
        let resultado = validar(app).and_then(|e| {
            let nombre = e.nombre.clone();
            app.registro.insertar(e).map(|()| nombre)
        });
        match resultado {
            Ok(nombre) => {
                app.set_result(&format!(
                    "Estudiante registrado: {nombre}. Total: {}.",
                    app.registro.total()
                ));
                app.limpiar_campos();
            }
            Err(mensaje) => app.set_error(&mensaje),
        }
    }

    app.mostrar_feedback(ui);
}

pub(crate) fn parsear_matricula(texto: &str) -> Result<u32, String> {
    let texto = texto.trim();
    if texto.len() != 5 || !texto.chars().all(|c| c.is_ascii_digit()) {
        return Err("La matrícula debe tener exactamente 5 dígitos.".to_owned());
    }
    let matricula: u32 = texto
        .parse()
        .map_err(|_| "Matrícula inválida.".to_owned())?;
    if !(MATRICULA_MIN..=MATRICULA_MAX).contains(&matricula) {
        return Err("La matrícula debe estar entre 10000 y 99999.".to_owned());
    }
    Ok(matricula)
}

pub(crate) fn parsear_grado(texto: &str) -> Result<u8, String> {
    texto
        .trim()
        .parse::<u8>()
        .ok()
        .filter(|g| (GRADO_MIN..=GRADO_MAX).contains(g))
        .ok_or_else(|| format!("El grado debe ser un entero entre {GRADO_MIN} y {GRADO_MAX}."))
}

pub(crate) fn parsear_promedio(texto: &str) -> Result<f64, String> {
    texto
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|p| p.is_finite() && (0.0..=100.0).contains(p))
        .ok_or_else(|| "El promedio debe ser un número entre 0 y 100.".to_owned())
}

fn validar(app: &EstudiantesApp) -> Result<Estudiante, String> {
    if app.nombre.trim().is_empty()
        || app.matricula.trim().is_empty()
        || app.grado.trim().is_empty()
        || app.promedio.trim().is_empty()
    {
        return Err("Todos los campos son obligatorios.".to_owned());
    }
    Ok(Estudiante {
        nombre: app.nombre.trim().to_owned(),
        matricula: parsear_matricula(&app.matricula)?,
        grado: parsear_grado(&app.grado)?,
        promedio: parsear_promedio(&app.promedio)?,
    })
}
