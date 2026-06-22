use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("布局模型", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("布局模型演示");

        ui.separator();
        ui.label(format!("可用空间: {:?}", ui.available_size()));

        ui.spacing_mut().item_spacing = egui::vec2(10.0, 5.0);

        ui.horizontal(|ui| {
            ui.label("水平排列:");
            ui.label("A");
            ui.label("B");
            ui.label("C");
        });

        ui.vertical_centered(|ui| {
            ui.heading("居中的标题");
            ui.label("这段文字也居中显示");
        });

        ui.separator();
        ui.label("Sense 演示:");
        let (resp, _painter) = ui.allocate_painter(egui::vec2(200.0, 50.0), egui::Sense::click());
        if resp.clicked() { ui.label("矩形区域被点击!"); }
        if resp.hovered() { ui.label("鼠标悬停中..."); }
    }
}
