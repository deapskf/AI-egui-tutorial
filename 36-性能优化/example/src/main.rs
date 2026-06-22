use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("性能优化", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { item_count: usize }

impl Default for MyApp { fn default() -> Self { Self { item_count: 100 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("性能优化演示");
        ui.add(egui::Slider::new(&mut self.item_count, 1..=5000).text("列表项数"));

        ui.separator();
        // 大量 Widget 渲染——如无分层/裁剪会导致性能问题
        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for i in 0..self.item_count.min(1000) {
                ui.label(format!("Item #{i}"));
            }
            if self.item_count > 1000 {
                ui.label(format!("... 还有 {} 项（已裁剪）", self.item_count - 1000));
            }
        });

        ui.separator();
        ui.label("提示: Sense::hover() 比 Sense::click() 更轻量");
        ui.label("不使用交互的区域应用 Sense::focusable_noninteractive()");
    }
}
