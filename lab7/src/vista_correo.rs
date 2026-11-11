use crate::app::{GimnasioApp, NARANJA, SPACE_MD, SUBMIT_BUTTON_SIZE};
use eframe::egui;

const ASUNTO_ADEUDO: &str = "Presenta un adeudo";
const CUERPO_ADEUDO: &str = "Estimado/a cliente:\n\n\
    Espero que este mensaje le encuentre muy bien.\n\n\
    Le escribo únicamente para realizar un seguimiento de cortesía respecto sus mensualidades, \
    la cual figura como pendiente en nuestros registros. Entendemos perfectamente que, con el ritmo \
    del día a día, estos detalles pueden pasarse por alto.\n\n\
    Le adjunto nuevamente el estado de cuenta para su comodidad. Si ya ha realizado el pago, por \
    favor ignore este correo y acepte nuestro agradecimiento.\n\n\
    Quedo a su disposición por si requiere asistencia con los métodos de pago o si desea que \
    revisemos algún detalle juntos.\n\n\
    Un cordial saludo,";

pub(crate) fn show(app: &mut GimnasioApp, ui: &mut egui::Ui) {
    ui.heading("Envío de correo electrónico a deudores");
    ui.label(
        egui::RichText::new(format!(
            "Asunto: {ASUNTO_ADEUDO} — deudores registrados: {}",
            app.arbol.deudores().len()
        ))
        .small()
        .weak(),
    );
    ui.add_space(SPACE_MD);

    if ui
        .add_sized(
            SUBMIT_BUTTON_SIZE,
            egui::Button::new("ENVIAR CORREO A DEUDORES"),
        )
        .clicked()
    {
        let deudores = app.arbol.deudores();
        let con_correo: Vec<String> = deudores.iter().filter_map(|c| c.correo.clone()).collect();
        app.sin_correo = deudores.len() - con_correo.len();
        app.enviada_a = con_correo;
        app.envio_realizado = true;
        app.set_result(&format!(
            "Correo enviado a {} deudores.",
            app.enviada_a.len()
        ));
    }

    ui.add_space(SPACE_MD);
    if app.envio_realizado {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .show(ui, |ui| mostrar_correo(app, ui));
    }

    app.mostrar_feedback(ui);
}

fn mostrar_correo(app: &GimnasioApp, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new(ASUNTO_ADEUDO).strong());
    ui.separator();

    egui::ScrollArea::vertical()
        .id_salt("cuerpo_adeudo")
        .max_height(180.0)
        .show(ui, |ui| {
            ui.label(CUERPO_ADEUDO);
        });

    ui.separator();
    ui.label(egui::RichText::new("Destinatarios:").strong());
    if app.enviada_a.is_empty() {
        ui.label("Ningún deudor cuenta con correo electrónico.");
    } else {
        for destinatario in &app.enviada_a {
            ui.label(format!("• {destinatario}"));
        }
    }

    ui.separator();
    let aviso = format!(
        "Advertencia: {} cliente(s) deudores no recibirán este correo por no contar con correo electrónico.",
        app.sin_correo
    );
    ui.label(egui::RichText::new(aviso).color(NARANJA).strong());
}
