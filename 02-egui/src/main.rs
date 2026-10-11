use eframe::egui;

#[derive(Default)]
struct App {
    todos: Vec<(String, bool)>,
    input: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("TODO (egui)");
            ui.horizontal(|ui| {
                let r = ui.text_edit_singleline(&mut self.input);
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if (ui.button("追加").clicked() || enter) && !self.input.trim().is_empty() {
                    self.todos.push((self.input.trim().to_string(), false));
                    self.input.clear();
                    r.request_focus();
                }
            });
            let mut remove = None;
            for (i, (title, done)) in self.todos.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.checkbox(done, title.as_str());
                    if ui.button("削除").clicked() {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                self.todos.remove(i);
            }
            ui.separator();
            ui.label(format!("残り {} 件", self.todos.iter().filter(|t| !t.1).count()));
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "TODO",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
