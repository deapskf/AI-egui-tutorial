use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("自定义Widget", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { rating: u8 }

impl Default for MyApp { fn default() -> Self { Self { rating: 3 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("自定义星级评分组件");

        // 星级评分
        ui.horizontal(|ui| {
            for i in 0..5 {
                let (resp, painter) = ui.allocate_painter(egui::vec2(30.0, 30.0), egui::Sense::click());
                let filled = i < self.rating;
                let color = if filled { egui::Color32::YELLOW } else { egui::Color32::DARK_GRAY };
                painter.circle(resp.rect.center(), 12.0, color, egui::Stroke::new(1.0, egui::Color32::WHITE));
                if resp.clicked() { self.rating = i + 1; }
            }
        });

        ui.label(format!("评分: {}/5", self.rating));

        if ui.button("重置").clicked() { self.rating = 0; }
    }
}
