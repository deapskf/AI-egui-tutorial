use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("图标库", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("图标演示");

        // egui 支持 emoji 作为内置 icons，也可集成第三方图标库如 egui_phosphor
        ui.horizontal(|ui| {
            if ui.button("💾 保存").clicked() { }
            if ui.button("➕ 新建").clicked() { }
            if ui.button("🔍 搜索").clicked() { }
            if ui.button("⚙ 设置").clicked() { }
        });

        ui.separator();
        ui.label("提示: egui_phosphor crate (0.34) 提供专业图标集");
        ui.label("用法: egui_phosphor::regular::FLOPPY_DISK 等");
        ui.label("参考 crates.io/crates/egui_phosphor");
    }
}
