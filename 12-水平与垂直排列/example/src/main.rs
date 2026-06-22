use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("水平与垂直排列", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { tags: Vec<String> }

impl Default for MyApp {
    fn default() -> Self { Self { tags: vec!["Rust".into(),"EGUI".into(),"GUI".into(),"Tutorial".into(),"中文".into(),"Hello".into(),"World".into()] } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("水平排列与自动换行");
        ui.horizontal_wrapped(|ui| {
            for tag in &self.tags {
                ui.add(egui::Button::new(tag).small());
            }
        });

        ui.separator();
        ui.heading("启用/禁用区域");
        ui.add_enabled(false, egui::Button::new("禁用按钮"));
        ui.add_enabled(true, egui::Button::new("正常按钮"));

        ui.separator();
        ui.heading("手动分配区域");
        let w = ui.available_width();
        let left = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(w * 0.4, 60.0));
        ui.scope_builder(egui::UiBuilder::new().max_rect(left), |ui| {
            ui.label("左侧 40%");
        });
        let right = egui::Rect::from_min_size(egui::pos2(ui.cursor().min.x + w * 0.4 + 4.0, ui.cursor().min.y - 60.0), egui::vec2(w * 0.6 - 4.0, 60.0));
        ui.scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
            ui.label("右侧 60%");
        });
    }
}
