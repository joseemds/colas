use eframe::{egui, App};

#[derive(Default)]
struct ColasApp;

impl App for ColasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Head");
            ui.label("Hello there")
        });
    }
}

fn main() {
    eframe::run_native(
        "Colas",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<ColasApp>::default()),
    )
    .unwrap();
}
